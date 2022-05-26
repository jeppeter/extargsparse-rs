
use super::key::{ExtKeyParse,KEYWORD_BOOL,KEYWORD_VALUE,KEYWORD_STRING,KEYWORD_HELP,KEYWORD_ARGS,KEYWORD_DICT,KEYWORD_INT,KEYWORD_FLOAT,KEYWORD_LIST,KEYWORD_JSONFILE,KEYWORD_COUNT,Nargs,KEYWORD_ATTR,KeyAttr};
use super::options::{ExtArgsOptions,OPT_SCREEN_WIDTH,OPT_EPILOG,OPT_DESCRIPTION,OPT_PROG,OPT_USAGE,OPT_VERSION};
use super::logger::{extargs_debug_out};
use super::{extargs_assert,extargs_log_warn,extargs_log_trace};
use super::funccall::{ExtArgsMatchFuncMap};
use super::helpsize::{HelpSize,CMD_NAME_SIZE,CMD_HELP_SIZE,OPT_NAME_SIZE,OPT_EXPR_SIZE,OPT_HELP_SIZE};

use serde_json::{Value};
use std::env;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Clone)]
struct InnerParserCompat {
    pub keycls :Option<ExtKeyParse>,
    pub cmdname :String,
    pub cmdopts :Vec<ExtKeyParse>,
    pub subcmds :Vec<Rc<RefCell<InnerParserCompat>>>,
    pub helpinfo :String,
    pub callfunction :String,
    pub screenwidth :i32,
    pub epilog :String,
    pub description :String,
    pub prog :String,
    pub usage :String,
    pub version :String,
}



impl InnerParserCompat {

    pub (crate) fn new(_cls :Option<ExtKeyParse> , _opt :Option<ExtArgsOptions>) -> InnerParserCompat {
        let mut retc :InnerParserCompat = InnerParserCompat {
            keycls : None,
            cmdname : "".to_string(),
            cmdopts : Vec::new(),
            subcmds : Vec::new(),
            helpinfo : "".to_string(),
            callfunction : "".to_string(),
            screenwidth : 80,
            epilog : "".to_string(),
            description : "".to_string(),
            prog : "".to_string(),
            usage : "".to_string(),
            version : "".to_string(),
        };
        let mut tmps :String;
        let jsonv :Value;
        let mut isopt :bool = false;

        if _cls.is_some() {
            let c :ExtKeyParse = _cls.unwrap();
            extargs_assert!(c.is_cmd(),"{} must be cmd", c.string());
            retc.keycls = Some(c.clone());
            retc.cmdname = c.cmd_name();
            /*no cmdopts no subcommands*/       
            retc.helpinfo = format!("{} handler", retc.cmdname);
            tmps = c.help_info();
            if tmps.len() > 0 {
                retc.helpinfo = tmps;
            }
            tmps = c.func_name();
            if tmps.len() > 0 {
                retc.callfunction = tmps;
            }
        } else {
            tmps = format!("{{}}");
            jsonv = serde_json::from_str(&tmps).unwrap();
            match ExtKeyParse::new("","main",&jsonv,false,false,false,"--","-",false) {
                Ok(_cv) => {
                    retc.keycls = Some(_cv);
                },
                Err(_e) => {
                    panic!("can not parse [{}] error[{:?}]", tmps,_e);
                }
            }
        }
        retc.screenwidth = 80;

        if _opt.is_some() {
            isopt = true;
        }

        if isopt  {
            let optc = _opt.as_ref().unwrap().clone();
            if optc.get_value(OPT_SCREEN_WIDTH).is_some() {
                retc.screenwidth = optc.get_int(OPT_SCREEN_WIDTH);  
            }
            retc.epilog = optc.get_string(OPT_EPILOG);
            retc.description = optc.get_string(OPT_DESCRIPTION);
            retc.prog = optc.get_string(OPT_PROG);
            retc.usage = optc.get_string(OPT_USAGE);
            retc.version = optc.get_string(OPT_VERSION);        
        }

        if retc.screenwidth < 40 {
            retc.screenwidth = 40;
        }

        retc
    }
    fn get_help_info(&self,_keycls :Option<&ExtKeyParse>,mapv :ExtArgsMatchFuncMap) -> String {
        extargs_assert!(_keycls.is_some(), "must no be null");
        let keycls = _keycls.unwrap();
        let oattr :Option<KeyAttr> = keycls.get_keyattr(KEYWORD_ATTR);
        let mut rets :String = "".to_string();
        if oattr.is_some() {
            let attr = oattr.unwrap();
            let hlp = attr.get_attr("opthelp");
            if hlp.len() > 0 {
                let funchelp = mapv.get_help_func(&hlp);
                if funchelp.is_some() {
                    let callf = funchelp.unwrap();
                    return callf(keycls);
                }
                extargs_log_warn!("can not find function [{}] for opthelp", hlp);
            }
        }

        if keycls.type_name() == KEYWORD_BOOL {
            if keycls.get_bool_v(KEYWORD_VALUE) == true {
                rets.push_str(&(format!("{} set false default(True)", keycls.opt_dest())));
            } else {
                rets.push_str(&(format!("{} set true default(False)", keycls.opt_dest())));
            }
        } else if keycls.type_name() == KEYWORD_STRING && keycls.get_string_v(KEYWORD_VALUE) == "+" {
            if keycls.is_flag() == true {
                rets.push_str(&(format!("{} inc", keycls.opt_dest())));
            } else {
                extargs_assert!(false == true,"cmd({}) can not set value({:?})", keycls.cmd_name(), keycls.get_string_v(KEYWORD_STRING));
            }
        } else if keycls.type_name() == KEYWORD_HELP {
            rets.push_str(&(format!("to display this help information")));
        } else {
            if keycls.is_flag() == true {
                if keycls.type_name() == KEYWORD_STRING {
                    match keycls.value() {
                        Value::String(_s) => {
                            rets.push_str(&(format!("{} set default {}",keycls.opt_dest(),_s)));
                        },
                        Value::Null => {
                            rets.push_str(&(format!("{} set default null",keycls.opt_dest())));
                        },
                        _ => {
                            extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value());
                        }
                    }
                } else if keycls.type_name() == KEYWORD_INT {
                    match keycls.value() {
                        Value::Number(_n) => {
                            if _n.is_i64() {
                                rets.push_str(&(format!("{} set default {}",keycls.opt_dest(),_n.as_i64().unwrap())));  
                            } else if _n.is_u64() {
                                rets.push_str(&(format!("{} set default {}",keycls.opt_dest(),_n.as_u64().unwrap())));  
                            } else {
                                extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value()); 
                            }
                        },
                        _ => {
                            extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value());
                        }
                    }
                } else if keycls.type_name() == KEYWORD_FLOAT {
                    match keycls.value() {
                        Value::Number(_n) => {
                            if _n.is_f64() {
                                rets.push_str(&(format!("{} set default {}",keycls.opt_dest(),_n.as_f64().unwrap())));  
                            } else {
                                extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value()); 
                            }
                        },
                        _ => {
                            extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value());
                        }
                    }

                } else if keycls.type_name() == KEYWORD_LIST {
                    let mut c :String = "[".to_string();
                    let mut idx :i32 = 0;
                    match keycls.value() {
                        Value::Array(_a) => {
                            for i in _a.iter() {
                                match i {
                                    Value::String(_s) => {
                                        if idx > 0 {
                                            c.push_str(",");
                                        }
                                        c.push_str(&format!("{}",_s));
                                        idx += 1;
                                    },
                                    _ => {
                                        extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value());
                                    }                        
                                }
                            }
                        },
                        _ => {
                            extargs_assert!(1 != 1, "can not get value type {:?}", keycls.value());
                        }                        
                    }
                    c.push_str("]");
                    rets.push_str(&(format!("{} set default {}",keycls.opt_dest(),c)));  
                } else if keycls.type_name() == KEYWORD_JSONFILE {
                    rets.push_str(&(format!("{} set default null",keycls.opt_dest())));  
                } else if keycls.type_name() == KEYWORD_COUNT {
                    rets.push_str(&format!("count set default 0"));
                }                
            } else {
                rets.push_str(&(format!("{} command exec", keycls.cmd_name())));
            }
        }

        rets
    }

    fn get_opt_help_optname(&self,_opt :Option<&ExtKeyParse>) -> String {
        let mut rets :String = "".to_string();
        if _opt.is_some() {
            let opt = _opt.unwrap();
            rets.push_str(&format!("{}",opt.long_opt()));
            if opt.short_opt().len() > 0 {
                rets.push_str(&format!("|{}",opt.short_opt()));
            }
        }
        rets
    }

    fn get_opt_help_optexpr(&self,_opt :Option<&ExtKeyParse>) -> String {
        let mut rets :String = "".to_string();
        if _opt.is_some() {
            let opt = _opt.unwrap();
            if opt.type_name() != KEYWORD_BOOL &&  opt.type_name() != KEYWORD_ARGS && 
            opt.type_name() != KEYWORD_DICT &&   opt.type_name() != KEYWORD_HELP {
                rets = opt.var_name();
                rets = rets.replace("-","_");
            }
        }
        rets
    }

    fn get_opt_help_opthelp(&self,_opt :Option<&ExtKeyParse>,mapv :ExtArgsMatchFuncMap) -> String {
        return self.get_help_info(_opt,mapv);
    }

    fn get_cmd_help_cmdname(&self) -> String {
        let mut rets :String = "".to_string();
        if self.cmdname.len() > 0 {
            rets = format!("[{}]",self.cmdname);
        }
        rets
    }

    fn get_cmd_help_cmdhelp(&self) -> String {
        let mut rets :String = "".to_string();
        if self.helpinfo.len() > 0 {
            rets = format!("{}",self.helpinfo);
        }
        rets
    }

    pub (crate) fn get_help_size(&self,hs :HelpSize, recursive :i32,mapv :ExtArgsMatchFuncMap) {
        hs.set_value(CMD_NAME_SIZE,self.get_cmd_help_cmdname().len() as i32 + 1);
        hs.set_value(CMD_HELP_SIZE,self.get_cmd_help_cmdhelp().len() as i32 + 1);

        for curopt in self.cmdopts.iter() {
            if curopt.type_name() == KEYWORD_ARGS {
                continue;
            }
            let copt = Some(curopt);
            hs.set_value(OPT_NAME_SIZE, self.get_opt_help_optname(copt).len() as i32 + 1);
            hs.set_value(OPT_EXPR_SIZE, self.get_opt_help_optexpr(copt).len() as i32 + 1);
            hs.set_value(OPT_HELP_SIZE, self.get_opt_help_opthelp(copt,mapv.clone()).len() as i32 + 1);
        }

        if recursive != 0 {
            for cursub in self.subcmds.iter() {
                if recursive > 0 {
                    cursub.borrow().get_help_size(hs.clone(),recursive - 1,mapv.clone());
                } else {
                    cursub.borrow().get_help_size(hs.clone(),recursive,mapv.clone());
                }
            }
        }

        for cursub in self.subcmds.iter() {
            hs.set_value(CMD_NAME_SIZE,cursub.borrow().get_cmd_help_cmdname().len() as i32 + 1);
            hs.set_value(CMD_HELP_SIZE,cursub.borrow().get_cmd_help_cmdhelp().len() as i32 + 1);
        }
    }

    fn get_indent_string(&self,s :String, indentsize :i32 , maxsize :i32) -> String {
        let mut curs :String = "".to_string();
        let mut rets :String = "".to_string();
        let ncurs :String;
        let mut i :usize;
        let mut j :usize;

        i = 0;
        while i < indentsize as usize {
            curs.push(' ');
            i += 1;
        }

        let bs = s.as_bytes();
        j = 0;
        while j < bs.len() {
            if (bs[j] == ' ' as u8 || bs[j] == '\t' as u8 ) && curs.len() > maxsize as usize {
                rets.push_str(&(format!("{}\n",curs)));
                curs = "".to_string();
                i = 0;
                while i < indentsize as usize {
                    curs.push(' ');
                    i += 1;
                }
                j += 1;
                continue;
            }
            curs.push( bs[j] as char);
            j += 1;
        }

        ncurs = format!("{}",curs.trim_start());
        if ncurs.len() > 0 {
            rets.push_str(&(format!("{}",curs.trim())));
        }
        rets
    }

    pub (crate) fn get_help_info_ex(&self,hs :HelpSize,parentcmds :Vec<Rc<RefCell<InnerParserCompat>>>,mapv :ExtArgsMatchFuncMap) -> String {
        let mut rets :String = "".to_string();
        self.get_help_size(hs.clone(),0,mapv.clone());
        if self.usage.len() > 0 {
            rets.push_str(&(format!("{}",self.usage)));
        } else {
            if parentcmds.len() > 0 {
                let rootcmds = &(parentcmds[0].borrow());
                /*copy code for safe not free*/
                if rootcmds.prog.len() > 0 {
                    rets.push_str(&(format!("{}",rootcmds.prog)));
                } else {
                    for arg in env::args() {
                        rets.push_str(&(format!("{}", arg)));
                        break;
                    }
                }
            } else {
                let rootcmds = &self;
                if rootcmds.prog.len() > 0 {
                    rets.push_str(&(format!("{}",rootcmds.prog)));
                } else {
                    for arg in env::args() {
                        rets.push_str(&(format!("{}", arg)));
                        break;
                    }
                }
            }


            if parentcmds.len() > 0 {
                for c in parentcmds.iter() {
                    rets.push_str(&(format!(" {}", c.borrow().cmdname)));
                }
            }

            rets.push_str(&(format!(" {}",self.cmdname)));
            if self.helpinfo.len() > 0 {
                rets.push_str(&(format!(" {}",self.helpinfo)));
            } else {
                if self.cmdopts.len() > 0 {
                    rets.push_str(&(format!(" [OPTIONS]")));
                }

                if self.subcmds.len() > 0 {
                    rets.push_str(&(format!(" [SUBCOMMANDS]")));
                }

                for curopt in self.cmdopts.iter() {
                    if curopt.type_name() == KEYWORD_ARGS {
                        match curopt.get_nargs_v() {
                            Nargs::Argtype(s) => {
                                if s == "+" {
                                    rets.push_str(&(format!(" args...")));
                                } else if s == "*" {
                                    rets.push_str(&(format!(" [args...]")));
                                } else if s == "?" {
                                    rets.push_str(&(format!(" arg")));
                                }
                            },
                            Nargs::Argnum(n) => {
                                if n > 1 {
                                    rets.push_str(&(format!(" args...")));
                                } else if n == 1 {
                                    rets.push_str(&(format!(" arg")));
                                }
                            },
                        }
                    } 
                }
            }
            rets.push_str(&(format!("\n")));
        }
        

        if self.description.len() > 0 {
            rets.push_str(&(format!("{}\n",self.description)));
        }

        extargs_log_trace!("hs [{}]",hs.string());

        rets.push_str(&(format!("\n")));
        if self.cmdopts.len() > 0 {
            rets.push_str(&format!(" [OPTIONS]\n"));

            for curopt in self.cmdopts.iter() {
                let mut curs :String = "".to_string();
                if curopt.type_name() == KEYWORD_ARGS {
                    continue;
                }
                let optname = self.get_opt_help_optname(Some(curopt));
                let optexpr = self.get_opt_help_optexpr(Some(curopt));
                let opthelp = self.get_opt_help_opthelp(Some(curopt),mapv.clone());
                let namesize = hs.get_value(OPT_NAME_SIZE) as usize;
                let exprsize = hs.get_value(OPT_EXPR_SIZE) as usize;
                let helpsize = hs.get_value(OPT_HELP_SIZE) as usize;
                extargs_log_trace!("namesize [{}] exprsize [{}] helpsize [{}]",namesize,exprsize,helpsize);
                extargs_log_trace!("optname [{}] optexpr [{}] opthelp [{}]", optname, optexpr, opthelp);
                curs.push_str(&(format!("    ")));
                curs.push_str(&(format!("{:<namesize$} {:<exprsize$} {:<helpsize$}\n",optname,optexpr,opthelp)));
                if curs.len() < self.screenwidth as usize {
                    rets.push_str(&curs);
                } else {
                    curs = "".to_string();
                    curs.push_str(&(format!("    ")));
                    curs.push_str(&format!("{:<namesize$} {:<exprsize$}",optname,optexpr));
                    rets.push_str(&(format!("{}\n",curs)));
                    if self.screenwidth > 60 {
                        rets.push_str(&(self.get_indent_string(opthelp, 20,self.screenwidth)));
                    } else {
                        rets.push_str(&(self.get_indent_string(opthelp,15,self.screenwidth)));
                    }
                    rets.push_str("\n");
                }
            }
        }

        if self.subcmds.len() > 0 {
            rets.push_str(&(format!("\n")));
            rets.push_str(&(format!("[SUBCOMMANDS]\n")));

            for curcmd in self.subcmds.iter() {
                let cmdname = curcmd.borrow().get_cmd_help_cmdname();
                let cmdhelp = curcmd.borrow().get_cmd_help_cmdhelp();
                let mut curs :String = "".to_string();
                let namesize = hs.get_value(CMD_NAME_SIZE)  as usize;
                let helpsize = hs.get_value(CMD_HELP_SIZE) as usize;

                curs.push_str(&(format!("    ")));
                curs.push_str(&(format!("{:<namesize$} {:<helpsize$}",cmdname,cmdhelp)));
                if curs.len() < self.screenwidth as usize {
                    rets.push_str(&(format!("{}\n",curs)));
                } else {
                    curs = "".to_string();
                    curs.push_str(&format!("    "));
                    curs.push_str(&format!("{:<namesize$}", cmdname));
                    rets.push_str(&(format!("{}\n",curs)));
                    if self.screenwidth > 60 { 
                        rets.push_str(&(self.get_indent_string(cmdhelp,20, self.screenwidth)));
                    } else {
                        rets.push_str(&(self.get_indent_string(cmdhelp,15,self.screenwidth)));
                    }
                }
            }
        }

        if self.epilog.len() > 0 {
            rets.push_str(&format!("\n"));
            rets.push_str(&(format!("\n{}\n",self.epilog)));
        }
        extargs_log_trace!("{}",rets);

        rets
    }

    pub (crate) fn string(&self) -> String {
        let mut rets :String = "".to_string();
        let mut i :i32;
        rets.push_str(&(format!("@{}|",self.cmdname)));
        if self.keycls.is_some() {
            let k = self.keycls.as_ref().unwrap();
            rets.push_str(&format!("{}|",k.string()));
        } else {
            rets.push_str(&(format!("nil|")));
        }

        if self.subcmds.len() > 0 {
            rets.push_str(&(format!("subcommands[{}]<",self.subcmds.len())));
            i = 0;
            for curcmd in self.subcmds.iter() {
                if i > 0 {
                    rets.push_str(&(format!(",")));
                }
                rets.push_str(&(format!("{}",curcmd.borrow().cmdname)));
                i += 1;
            }
            rets.push_str(&(format!(">")));
        }

        if self.cmdopts.len() > 0 {
            rets.push_str(&(format!("cmdopts[{}]<",self.cmdopts.len())));
            for curopt in self.cmdopts.iter() {
                rets.push_str(&format!("{}",curopt.string()));
            }
            rets.push_str(&(format!(">")));
        }

        rets
    }
}

#[derive(Clone)]
pub struct ParserCompat {
    innerrc : Rc<RefCell<InnerParserCompat>>,
}

impl ParserCompat {
    pub (crate) fn new(_cls :Option<ExtKeyParse> , _opt :Option<ExtArgsOptions>) -> ParserCompat {
        let k = InnerParserCompat::new(_cls,_opt);
        ParserCompat {
            innerrc : Rc::new(RefCell::new(k)),
        }
    }

    pub (crate) fn cmd_name(&self) -> String {
        return format!("{}",self.innerrc.borrow().cmdname);
    }

    pub (crate) fn sub_cmds(&self) -> Vec<ParserCompat> {
        let mut retv :Vec<ParserCompat> = Vec::new();
        for v in self.innerrc.borrow().subcmds.iter() {
            retv.push(ParserCompat{
                innerrc : v.clone(),
            });
        }
        retv
    }

    pub (crate) fn get_keycls(&self) -> Option<ExtKeyParse> {
        if self.innerrc.borrow().keycls.is_some() {
            let bk = self.innerrc.borrow();
            let k = bk.keycls.as_ref().unwrap();
            return Some(k.clone());
        }
        return None;
    }

    pub (crate) fn get_cmdopts(&self) -> Vec<ExtKeyParse> {
        let mut retv :Vec<ExtKeyParse> = Vec::new();
        for v in self.innerrc.borrow().cmdopts.iter() {
            retv.push(v.clone());
        }
        retv
    }

    pub (crate) fn push_cmdopts(&self, keycls :ExtKeyParse) {
        self.innerrc.borrow_mut().cmdopts.push(keycls);
        return;
    }

    pub (crate) fn push_subcmds(&self ,input :ParserCompat) {
        let mut v = self.innerrc.borrow_mut();
        v.subcmds.push(input.innerrc);
        return;
    }

    pub (crate) fn string(&self) -> String {
        return self.innerrc.borrow().string();
    }

    pub (crate) fn get_help_info_ex(&self,hs :HelpSize, cmdpaths :Vec<ParserCompat>, mapv :ExtArgsMatchFuncMap) -> String {
        let mut innerpaths :Vec<Rc<RefCell<InnerParserCompat>>> = Vec::new();

        for v in cmdpaths {
            innerpaths.push(v.innerrc.clone());
        }
        return self.innerrc.borrow().get_help_info_ex(hs,innerpaths,mapv);
    }
}