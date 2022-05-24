
use std::env;
use std::fs;
use std::thread;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::process;
use tempfile::TempDir;
use std::ffi::OsStr;
use std::time;
use std::collections::HashMap;


use lazy_static::lazy_static;
use extargsparse_codegen::{extargs_load_commandline};


use super::{extargs_new_error,extargs_error_class,extargs_log_trace};
use super::parser::{ExtArgsParser};
use super::options::{ExtArgsOptions};
use super::logger::{extargs_debug_out};
use super::key::{ExtKeyParse,KEYWORD_HELP,KEYWORD_JSONFILE,KEYWORD_PREFIX,KEYWORD_LIST,KEYWORD_STRING,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_BOOL,KEYWORD_ARGS,KEYWORD_COUNT,KEYWORD_SUBNARGS};
use super::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};


#[derive(Debug,Clone)]
pub (crate) struct FuncComposer {
    funcstr :String,
    innerstr : String,
}

#[allow(dead_code)]
impl FuncComposer {
    pub fn new() -> FuncComposer {
        FuncComposer {
            funcstr : "".to_string(),
            innerstr : "".to_string(),
        }
    }

    pub fn add_code(&mut self, code :&str) {
        self.funcstr.push_str(code);
        self.funcstr.push_str("\n");
        return;
    }

    pub fn add_inner(&mut self, code :&str) {
        self.innerstr = "".to_string();
        self.innerstr.push_str(code);
        return;
    }


    pub fn get_func(&self) -> String {
        return format!("{}",self.funcstr);
    }

    pub fn get_extargs_map_func(&self) -> String {
        let mut rets :String = "".to_string();
        rets.push_str("#[extargs_map_function(");
        rets.push_str(&self.innerstr);
        rets.push_str(")]");
        return rets;
    }
}

pub (crate) struct Chdir {
    origdir :String,
    curdir :String,
}

impl Chdir {
    pub fn new() -> Chdir {
        let curdir = env::current_dir().unwrap().display().to_string();
        Chdir {
            origdir : curdir,
            curdir : "".to_string(),
        }
    }

    pub fn chdir(&mut self,ndir :&str) -> Result<(),Box<dyn Error>> {
        let np = Path::new(ndir);
        self.curdir = format!("{}",ndir);
        env::set_current_dir(&np)?;
        Ok(())
    }
}

impl Drop for Chdir {
    fn drop(&mut self) {
        if self.origdir.len() > 0 {
            let np = Path::new(&self.origdir);
            let _ = env::set_current_dir(&np).unwrap();
        }
        self.origdir = "".to_string();
        self.curdir = "".to_string();
        return;
    }
}

extargs_error_class!{ExtArgsDirError}

#[derive(Debug)]
pub (crate) struct ExtArgsDir {
    srcdir : String,
    workdir :String,
    gendir :String, 
    tdir : TempDir,
    writed : bool,
    exename : String,
}

#[cfg(debug_assertions)]
lazy_static!{
    static ref RUST_RUN_MODE :String = {
        format!("debug")
    };
}

#[cfg(not(debug_assertions))]
lazy_static!{
    static ref RUST_RUN_MODE :String = {
        format!("release")
    };
}

lazy_static!{
    static ref IS_WINDOWS_MODE :bool = {
        let mut retv :bool = false;
        if env::consts::OS == "windows"  {
            retv = true;
        }
        retv
    };
    static ref PATH_SPLIT :char = {
        let mut retv :char = '/';
        if *IS_WINDOWS_MODE {
            retv = '\\';
        }
        retv
    };
}

impl ExtArgsDir {
    pub fn new(exename :&str,workdir :&str,gendir :&str) -> ExtArgsDir {
        let mut retv :ExtArgsDir = ExtArgsDir{
            srcdir : "".to_string(),
            workdir : format!("{}",workdir),
            gendir : format!("{}",gendir ),
            tdir : TempDir::new().unwrap(),
            writed : false,
            exename : format!("{}",exename),
        };
        let srcd = retv.tdir.path();
        retv.srcdir = format!("{}",srcd.display());
        retv
    }

    fn uc_first(&self,n :&str) -> String {
        let cv :Vec<char> = n.chars().collect();
        let mut cidx :i32 =0;
        let mut rets :String = "".to_string();
        let bv :Vec<char> = n.to_uppercase().chars().collect();
        for c in cv.iter() {
            if cidx == 0 {
                rets.push(bv[0]);
            } else {
                rets.push(*c);
            }
            cidx += 1;
        }
        return rets;
    }


    fn get_cmd_struct_name(&self, cmdname :&str) -> String {
        let mut rets :String = "".to_string();
        if cmdname.len() == 0 {
            rets.push_str("MainDataStruct");
        } else {
            let cv :Vec<&str> = cmdname.split(".").collect();
            for cs in cv.iter() {
                rets.push_str(&self.uc_first(cs));
            }
            rets.push_str("DataStruct");
        }
        return rets;
    }

    fn get_parser_struct(&self,tabs :i32 ,parser :ExtArgsParser, cmdname :&str) -> Result<String,Box<dyn Error>> {
        let mut rets :String = "".to_string();
        let subcmds :Vec<String>;
        let opts :Vec<ExtKeyParse>;
        let mut idx : i32 = 0;
        let strprefix :String;

        strprefix = self.get_cmd_struct_name(cmdname);

        subcmds = parser.get_sub_commands_ex(cmdname)?;
        for c in subcmds.iter() {           
            let mut curcmd :String = format!("{}",cmdname);

            if curcmd.len() > 0 {
                curcmd.push_str(".");
            }
            curcmd.push_str(&(format!("{}", c)));
            let curs = self.get_parser_struct(tabs , parser.clone(),&curcmd)?;
            rets.push_str("\n");
            rets.push_str(&curs);
        }

        opts = parser.get_cmd_opts_ex(cmdname)?;
        for o in opts.iter() {
            if o.is_flag() && o.type_name() != KEYWORD_HELP && o.type_name() != KEYWORD_JSONFILE && o.type_name() != KEYWORD_PREFIX {
                let tname :String;
                let kname :String;
                if idx == 0 {
                    rets.push_str("#[derive(ArgSet)]\n");
                    rets.push_str(&(format!("struct {} {{\n",strprefix)));
                }
                if o.type_name() == KEYWORD_LIST {
                    tname = format!("Vec<String>");
                } else if o.type_name() == KEYWORD_STRING {
                    tname = format!("String");
                } else if o.type_name() == KEYWORD_INT {
                    tname = format!("i64");
                } else if o.type_name() == KEYWORD_FLOAT {
                    tname = format!("f64");
                } else if o.type_name() == KEYWORD_BOOL {
                    tname = format!("bool");
                } else if o.type_name() == KEYWORD_ARGS {
                    tname = format!("Vec<String>");
                } else if o.type_name() == KEYWORD_COUNT {
                    tname = format!("i64");
                } else {
                    extargs_new_error!{ExtArgsDirError,"not supported type [{}]", o.type_name()}
                }

                if o.type_name() != KEYWORD_ARGS {
                    kname = format!("{}",o.var_name());
                } else {
                    if cmdname.len() > 0 {
                        kname = format!("{}",KEYWORD_SUBNARGS);
                    } else {
                        kname = format!("{}",KEYWORD_ARGS);
                    }
                }

                rets.push_str(&format!("    {} : {},\n", kname, tname));
                idx += 1;
            }
        }

        for c in subcmds.iter() {           
            if idx == 0 {
                rets.push_str("#[derive(ArgSet)]\n");
                rets.push_str(&(format!("struct {} {{\n",strprefix)));
            }

            let mut cname :String = "".to_string();
            let kname :String;
            if cmdname.len() > 0 {
                cname.push_str(cmdname);
                cname.push_str(".");
            }
            cname.push_str(c);

            kname = self.get_cmd_struct_name(&cname);

            rets.push_str(&format!("    {} : {},\n",c,kname));
            idx += 1;
        }

        if idx > 0 {
            rets.push_str("}\n");
        }
        Ok(rets)
    }

    fn format_priority(&self, priority :Option<Vec<i32>>) -> String {
        let mut rets :String = "None".to_string();

        if priority.is_some() {
            rets = "Some".to_string();
            rets.push_str("(vec![");
            let pv = priority.unwrap().clone();
            let mut idx :i32 = 0;
            for v in pv.iter() {
                if idx > 0 {
                    rets.push_str(",");
                }
                if *v == COMMAND_SET {
                    rets.push_str("COMMAND_SET");
                } else if *v == SUB_COMMAND_JSON_SET {
                    rets.push_str("SUB_COMMAND_JSON_SET");
                } else if *v == COMMAND_JSON_SET {
                    rets.push_str("COMMAND_JSON_SET");
                } else if *v == ENVIRONMENT_SET {
                    rets.push_str("ENVIRONMENT_SET");
                } else if *v == ENV_SUB_COMMAND_JSON_SET {
                    rets.push_str("ENV_SUB_COMMAND_JSON_SET");
                } else if *v == ENV_COMMAND_JSON_SET {
                    rets.push_str("ENV_COMMAND_JSON_SET");
                } else if *v == DEFAULT_SET {
                    rets.push_str("DEFAULT_SET");
                } else {
                    panic!("not valid priority");
                }
                idx += 1;
            }


            rets.push_str("])");
        }

        rets
    }

    fn format_imports(&self) -> String {
        let mut rets :String = "".to_string();
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use extargsparse_worker::{extargs_error_class,extargs_new_error};\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use extargsparse_worker::namespace::{NameSpaceEx};\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use extargsparse_worker::argset::{ArgSetImpl};\n");
        rets.push_str("use extargsparse_worker::parser::{ExtArgsParser};\n");
        rets.push_str("use extargsparse_worker::funccall::{ExtArgsParseFunc};\n");
        rets.push_str("use extargsparse_worker::options::{ExtArgsOptions};\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};\n");
        rets.push_str("\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use std::cell::RefCell;\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use std::sync::Arc;\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use std::error::Error;\n");
        rets.push_str("use std::boxed::Box;\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use regex::Regex;\n");
        rets.push_str("#[allow(unused_imports)]\n");
        rets.push_str("use std::any::Any;\n");
        rets.push_str("use lazy_static::lazy_static;\n");
        rets.push_str("use std::collections::HashMap;\n");
        rets.push_str("\n");
        rets.push_str("\n");

        rets
    }
    fn format_extargs_map_functions(&self,fcomposer :FuncComposer) -> String {
        let mut rets =  fcomposer.get_extargs_map_func();
        rets.push_str("\n");
        rets
    }

    fn format_cargo_toml(&self) -> String {
        let mut rets :String = "".to_string();
        rets.push_str("[package]\n");
        rets.push_str(&format!("name = \"{}\"\n",self.exename));
        rets.push_str("version = \"0.1.0\"\n");
        rets.push_str("edition = \"2018\"\n");
        rets.push_str("\n");
        rets.push_str("[dependencies]\n");
        rets.push_str(&format!("extargsparse_codegen = {{ path = \"{}\"}}\n",self.gendir.replace("\\","\\\\")));
        rets.push_str(&format!("extargsparse_worker = {{ path = \"{}\" }}\n", self.workdir.replace("\\","\\\\")));
        rets.push_str("regex = \"1\"\n");
        rets.push_str("lazy_static = \"^1.4.0\"\n");
        rets.push_str("\n");

        rets
    }

    fn write_cargo_toml(&self) -> Result<(),Box<dyn Error>> {
        let cargopath = self.tdir.path().join("Cargo.toml").display().to_string();
        let mut fp: fs::File = fs::File::create(&cargopath).unwrap();
        fp.write_all(self.format_cargo_toml().as_bytes())?;
        Ok(())
    }

    fn format_print_out(&self,parser :ExtArgsParser,cmdname :&str, piname :&str) -> String {
        let subcmds :Vec<String>;
        let mut rets :String = "".to_string();

        subcmds = parser.get_sub_commands_ex(cmdname).unwrap();
        for c in subcmds.iter() {
            let mut curcmd :String = "".to_string();
            curcmd.push_str(cmdname);
            if curcmd.len() > 0 {
                curcmd.push_str(".");
            }
            curcmd.push_str(c);
            rets.push_str(&self.format_print_out(parser.clone(),&curcmd,piname));
        }

        let opts = parser.get_cmd_opts_ex(cmdname).unwrap();
        let mut rename = format!("{}",cmdname);
        if rename.len() == 0 {
            rename = format!("main");
        }
        rets.push_str(&format!("    /* print out {} */\n",rename));
        for o in opts.iter() {
            if o.is_flag() && o.type_name() != KEYWORD_HELP && o.type_name() != KEYWORD_JSONFILE && 
            o.type_name() != KEYWORD_PREFIX {
                if o.type_name() != KEYWORD_ARGS {
                    if cmdname.len() > 0 {
                        if o.type_name() == KEYWORD_LIST {
                            rets.push_str(&format!("    println!(\"{}.{}.{}={{:?}}\", {}.borrow().{}.{});\n", piname,cmdname,o.var_name(), piname,cmdname,o.var_name()));   
                        } else {
                            rets.push_str(&format!("    println!(\"{}.{}.{}={{}}\", {}.borrow().{}.{});\n", piname,cmdname,o.var_name(), piname,cmdname,o.var_name()));     
                        }
                        
                    } else {
                        if o.type_name() == KEYWORD_LIST {
                            rets.push_str(&format!("    println!(\"{}.{}={{:?}}\", {}.borrow().{});\n", piname,o.var_name(), piname,o.var_name())); 
                        } else {
                            rets.push_str(&format!("    println!(\"{}.{}={{}}\", {}.borrow().{});\n", piname,o.var_name(), piname,o.var_name()));       
                        }
                        
                    }
                    
                } else {
                    let  argname :String;
                    if cmdname.len() > 0 {
                        argname = format!("{}",KEYWORD_SUBNARGS);
                        rets.push_str(&format!("    println!(\"{}.{}.{}={{:?}}\", {}.borrow().{}.{});\n", piname, cmdname,argname, piname,cmdname,argname));
                    } else {
                        argname = format!("{}",KEYWORD_ARGS);
                        rets.push_str(&format!("    println!(\"{}.{}={{:?}}\", {}.borrow().{});\n", piname,argname, piname,argname));
                    }
                    
                }
            }
        }
        rets.push_str("\n");
        rets
    }

    fn format_main(&self,optstr :&str,cmdstr :&str,printout : bool, parser :ExtArgsParser,priority :Option<Vec<i32>>,nsname :&str, piname :&str) -> String {
        let mut rets :String = "".to_string();

        rets.push_str("fn main() -> Result<(),Box<dyn Error>> {\n");
        rets.push_str("    let loads = r#\"");
        if cmdstr.len() > 0 {
            rets.push_str(cmdstr);  
        } else {
            rets.push_str("{}");
        }       
        rets.push_str("\"#;\n");
        rets.push_str("    let optstr = r#\"");
        if optstr.len() > 0 {
            rets.push_str(optstr);  
        } else {
            rets.push_str("{}");
        }       
        rets.push_str("\"#;\n");
        rets.push_str("    let optref = ExtArgsOptions::new(optstr)?;\n");
        rets.push_str(&format!("    let parser = ExtArgsParser::new(Some(optref.clone()),{})?;\n",self.format_priority(priority)));
        rets.push_str("    extargs_load_commandline!(parser,loads)?;\n");

        if printout {
            rets.push_str(&format!("    let {} = Arc::new(RefCell::new(MainDataStruct::new()));\n",piname));            
            rets.push_str(&format!("    let {} = parser.parse_commandline_ex(None,None,Some({}.clone()),None)?;\n",nsname,piname));
        } else {
            rets.push_str(&format!("    let {} = parser.parse_commandline_ex(None,None,None,None)?;\n",nsname));
        }

        if printout {
            rets.push_str(&self.format_print_out(parser.clone(),"",piname));
        }

        rets.push_str("    Ok(())");

        rets.push_str("}\n");
        rets
    }

    fn write_main_rs(&self,mainrs :&str) -> Result<(),Box<dyn Error>> {
        let mainrspath = self.tdir.path().join("src").join("main.rs").display().to_string();
        let maindirpath = self.tdir.path().join("src").display().to_string();
        fs::create_dir_all(&maindirpath)?;
        let mut fp: fs::File = fs::File::create(&mainrspath).unwrap();
        fp.write_all(mainrs.as_bytes())?;
        Ok(())
    }

    pub fn write_rust_code(&mut self,optstr :&str,cmdstr :&str, _addmode :Vec<String>,fcomposer :FuncComposer, priority :Option<Vec<i32>>,printout :bool, nsname :&str, piname :&str) -> Result<(),Box<dyn Error>> {
        self.write_cargo_toml()?;
        /*to get write main file*/
        let mut rets :String = "".to_string();
        rets.push_str(&(self.format_imports()));
        rets.push_str(&fcomposer.funcstr);

        let mut opt :Option<ExtArgsOptions> = None;
        if optstr.len() > 0 {
            let ov = ExtArgsOptions::new(optstr)?;
            opt = Some(ov.clone());
        }
        let mut inprior :Option<Vec<i32>> = None;
        if priority.is_some() {
            inprior =Some(priority.as_ref().unwrap().clone());
        }
        let parser :ExtArgsParser = ExtArgsParser::new(opt, inprior)?;
        /*now for the */
        extargs_load_commandline!(parser,cmdstr)?;
        /*now to get the string*/
        rets.push_str(&(self.get_parser_struct(0,parser.clone(),"").unwrap()));

        rets.push_str(&self.format_extargs_map_functions(fcomposer.clone()));
        inprior = None;
        if priority.is_some() {
            inprior =Some(priority.as_ref().unwrap().clone());
        }
        rets.push_str(&self.format_main(optstr,cmdstr,printout, parser.clone(),inprior,nsname,piname));

        self.write_main_rs(&rets)?;
        self.writed = true;
        Ok(())
    }

    fn compile_command(&self) -> Result<(),Box<dyn Error>> {
        let mut chd = Chdir::new();
        let cargoexe :String;
        let mode :String;
        chd.chdir(&self.srcdir)?;
        if *IS_WINDOWS_MODE {
            cargoexe = format!("cargo.exe");
        } else {
            cargoexe = format!("cargo");
        }

        mode = format!("--{}",*RUST_RUN_MODE);
        {
            let mut cmd = process::Command::new(&cargoexe);
            cmd.arg("build").arg(&mode);
            let mut chld = cmd.spawn()?;
            let mut waiting :i32 = 1;
            waiting = waiting;
            while waiting > 0 {
                match chld.try_wait() {
                    Ok(Some(status)) => {
                        if !status.success() {
                            let vargs :Vec<&OsStr> = cmd.get_args().collect();
                            extargs_new_error!{ExtArgsDirError,"wait {:?} exit {:?}", vargs, status}
                        }
                        waiting = 0;
                    },
                    Ok(None) => { thread::sleep(time::Duration::from_millis(50));},
                    Err(e) => {
                        let vargs :Vec<&OsStr> = cmd.get_args().collect();
                        extargs_new_error!{ExtArgsDirError,"wait {:?} error {:?}", vargs, e}
                    }
                }
            }           
        }
        Ok(())
    }

    #[allow(unused_assignments)]
    fn run_command(&self,envval :HashMap<String,String>,delvars :Vec<String>, args :Vec<String>) ->  Result<String,Box<dyn Error>>{
        let mut setenvs :HashMap<String,String> = env::vars().collect();
        let mut cont :i32 = 1;
        let mut chd :Chdir = Chdir::new();
        let mut rets :String = "".to_string();

        while cont > 0 {
            cont = 0;


            for (k,_) in setenvs.clone() {
                let mut delv :i32 = 0;
                for k2 in delvars.clone() {
                    if k2 == k {
                        delv = 1;
                        break;
                    }
                }

                if delv > 0 {
                    setenvs.remove(&k);
                    cont = 1;
                    break;
                }

                if k == "EXTARGSPARSE_LOGLEVEL" && cont == 0 {
                    cont = 1;
                    setenvs.remove(&k);
                    break;
                }
            }
        }

        for (k,v) in envval.clone() {
            setenvs.insert(k,v);
        }

        chd.chdir(&self.srcdir)?;
        let mut cname = format!("target{}{}{}{}",*PATH_SPLIT,*RUST_RUN_MODE,*PATH_SPLIT,self.exename);
        if *IS_WINDOWS_MODE {
            cname.push_str(".exe");
        }
        extargs_log_trace!("cname {} curdir {}", cname,self.srcdir);
        let mut cmd = process::Command::new(&cname);
        for d in args.clone() {
            cmd.arg(d);
        }

        let output = cmd.env_clear().envs(&setenvs).stdin(process::Stdio::null()).stdout(process::Stdio::piped()).stderr(process::Stdio::inherit()).output()?;
        if !output.status.success() {
        let vargs :Vec<&OsStr> = cmd.get_args().collect();
            extargs_new_error!{ExtArgsDirError,"run {:?} exit {:?}",vargs,output.status}
        }

        match std::str::from_utf8(&output.stdout) {
            Ok(v) => {
                rets = format!("{}",v);
            } ,
            Err(e) => {
                let vargs :Vec<&OsStr> = cmd.get_args().collect();
                extargs_new_error!{ExtArgsDirError,"run {:?} change buffer error {:?}", vargs, e}
            }
        }
        rets = rets;

        Ok(rets)
    }

    #[allow(unused_assignments)]
    pub fn compile_and_run(&self,envval :HashMap<String,String>, delenvval :Vec<String>, args :Vec<String>) -> Result<String,Box<dyn Error>> {
        let mut rets :String = "".to_string();
        if !self.writed {
            extargs_new_error!{ExtArgsDirError,"not writed yet"}
        }

        self.compile_command()?;
        extargs_log_trace!("compile over");
        rets = self.run_command(envval.clone(),delenvval.clone(),args.clone())?;
        Ok(rets)
    }
}