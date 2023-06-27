use rcommon::{WordList, WordDesc};
use ralias::r_alias_builtin;
use rbind::r_bind_builtin;
use rbreak::r_break_builtin;
use rbuiltin::r_builtin_builtin;
use rcaller::r_caller_builtin;
use rcd::r_cd_builtin;
//use rcmd::r_cmd_builtin;
use rcolon::r_colon_builtin;
//use command::command_builtin;
//use rcommon ::r__builtin;
use rcomplete::r_complete_builtin;
use rdeclare::r_declare_builtin;
use recho::r_echo_builtin;
use renable::r_enable_builtin;
use reval::r_eval_builtin;
use rexec::r_exec_builtin;
use rexit::r_exit_builtin;
use rfc::r_fc_builtin;
use rfg_bg::r_fg_builtin;
use rgetopts::r_getopts_builtin;
use rhash::r_hash_builtin;
use rhelp::r_help_builtin;
use rhistory::r_history_builtin;
use rjobs::r_jobs_builtin;
use rkill::r_kill_builtin;
use rmapfile::r_mapfile_builtin;
use rprintf::r_printf_builtin;
use rpushd::r_pushd_builtin;
use rread::r_read_builtin;
use rlet::r_let_builtin;
use rreturn::r_return_builtin;
use rset::r_set_builtin;
//use rsetattr::r_setattr_builtin;
use rshift::r_shift_builtin;
use rshopt::r_shopt_builtin;
use rsource::r_source_builtin;
use rsuspend::r_suspend_builtin;
use rtest::r_test_builtin;
use rtimes::r_times_builtin;
use rtrap::r_trap_builtin;
use rtype::r_type_builtin;
use rulimit::r_ulimit_builtin;
use rumask::r_umask_builtin;
use rwait::r_wait_builtin;

enum CMDType {
    AliasCmd,
    BindCmd,
    BreakCmd,
    BuiltinCmd,
    CallerCmd,
    CdCmd,
    ColonCmd,
    CommandCmd,
    CommonCmd,
    CompleteCmd,
    DeclareCmd,
    EchoCmd,
    EnableCmd,
    EvalCmd,
    ExecCmd,
    ExitCmd,
    FcCmd,
    FgbgCmd,
    GetoptsCmd,
    HashCmd,
    HelpCmd,
    HistoryCmd,
    JobsCmd,
    KillCmd,
    LetCmd,
    MapfileCmd,
    PrintfCmd,
    PushdCmd,
    ReadCmd,
    ReservedCmd,
    ReturnCmd,
    SetattrCmd,
    SetCmd,
    ShiftCmd,
    ShoptCmd,
    SourceCmd,
    SuspendCmd,
    TestCmd,
    TimesCmd,
    TrapCmd,
    TypeCmd,
    UlimitCmd,
    UmaskCmd,
    WaitCmd
}

  struct AliasComand ;
  impl CommandExec for AliasComand{
    fn  excute(&self,list : *mut WordList){
        unsafe {
            r_alias_builtin(list);
        }
    }
  }
  struct BindComand;
  impl CommandExec for BindComand{
    fn  excute(&self,list : *mut WordList){
        r_bind_builtin(list);
    }
}
  struct BreakComand;
  impl CommandExec for BreakComand{
    fn  excute(&self,list : *mut WordList){
        r_break_builtin(list);
    }
}
  struct BuiltinComand;
  impl CommandExec for BuiltinComand{
    fn  excute(&self,list : *mut WordList){
        r_builtin_builtin(list);
    }
}
  struct CallerComand;
  impl CommandExec for CallerComand{
    fn  excute(&self,list : *mut WordList){
        r_caller_builtin(list);
    }
}
  struct CdComand;
  impl CommandExec for CdComand{
    fn  excute(&self,list : *mut WordList){
        r_cd_builtin(list);
    }
}
  struct ColonComand;
  impl CommandExec for ColonComand{
    fn  excute(&self,list :*mut WordList){
       // r_colon_builtin(list);
    }
}
  struct CommandComand;
  impl CommandExec for CommandComand{
    fn  excute(&self,list : *mut WordList){
        //command_builtin(list);
    }
}
  struct CommonComand;
  impl CommandExec for CommonComand{
    fn  excute(&self,list : *mut WordList){
        unsafe {
            r_alias_builtin(list);
        }
    }
}
  struct CompleteComand; 
  impl CommandExec for CompleteComand{
    fn  excute(&self,list : *mut WordList){
        r_complete_builtin(list);
    }
}
  struct DeclareComand;
  impl CommandExec for DeclareComand{
    fn  excute(&self,list : *mut WordList){
        r_declare_builtin(list);
    }
}
  struct EchoComand;
  impl CommandExec for EchoComand{
    fn  excute(&self,list : *mut WordList){
        r_echo_builtin(list);
    }
}
  struct EnableComand;
  impl CommandExec for EnableComand{
    fn  excute(&self,list : *mut WordList){
      unsafe {
        r_enable_builtin(list);
      }  
    }
}
  struct EvalComand;
  impl CommandExec for EvalComand{
    fn  excute(&self,list : *mut WordList){
        r_eval_builtin(list);
    }
}
  struct ExecComand;
  impl CommandExec for ExecComand{
    fn  excute(&self,list : *mut WordList){
        r_exec_builtin(list);
    }
}
  struct ExitComand;
  impl CommandExec for ExitComand{
    fn  excute(&self,list : *mut WordList){
        r_exit_builtin(list);
    }
}
  struct FcComand;
  impl CommandExec for FcComand{
    fn  excute(&self,list : *mut WordList){
        r_fc_builtin(list);
    }
}
  struct FgbgComand;
  impl CommandExec for FgbgComand{
    fn  excute(&self,list : *mut WordList){
        r_fg_builtin(list);
    }
}
  struct GetoptsComand;
  impl CommandExec for GetoptsComand{
    fn  excute(&self,list : *mut WordList){
        r_getopts_builtin(list);
    }
}
  struct HashComand;
  impl CommandExec for HashComand{
    fn  excute(&self,list : *mut WordList){
       r_hash_builtin(list);
    }
}
  struct HelpComand;
  impl CommandExec for HelpComand{
    fn  excute(&self,list : *mut WordList){
        r_help_builtin(list);
    }
}
  struct HistoryComand;
  impl CommandExec for HistoryComand{
    fn  excute(&self,list : *mut WordList){
        r_history_builtin(list);
    }
}
  struct JobsComand;
  impl CommandExec for JobsComand{
    fn  excute(&self,list : *mut WordList){
        r_jobs_builtin(list);
    }
}
  struct KillComand;
  impl CommandExec for KillComand{
    fn  excute(&self,list : *mut WordList){
        r_kill_builtin(list);
    }
}
  struct LetComand;
  impl CommandExec for LetComand{
    fn  excute(&self,list : *mut WordList){
        r_let_builtin(list);
    }
}
  struct MapfileComand;
  impl CommandExec for MapfileComand{
    fn  excute(&self,list : *mut WordList){
        r_mapfile_builtin(list);
    }
}
  struct PrintfComand;
  impl CommandExec for PrintfComand{
    fn  excute(&self,list : *mut WordList){
        r_printf_builtin(list);
    }
}
  struct PushdComand;
  impl CommandExec for PushdComand{
    fn  excute(&self,list : *mut WordList){
        r_pushd_builtin(list);
    }
}
  struct ReadComand;
  impl CommandExec for ReadComand{
    fn  excute(&self,list : *mut WordList){
        r_read_builtin(list);
    }
}
  struct ReservedComand;
  impl CommandExec for ReservedComand{
    fn  excute(&self,list : *mut WordList){
        //r_reserve_builtin(list);
    }
}
  struct ReturnComand;
  impl CommandExec for ReturnComand{
    fn  excute(&self,list : *mut WordList){
        r_return_builtin(list);
    }
}
  struct SetattrComand;
  impl CommandExec for SetattrComand{
    fn  excute(&self,list : *mut WordList){
       //r_setattr_builtin(list);
       /*unkown enter which func */
    }  

}
  struct SetComand;
  impl CommandExec for SetComand{
    fn  excute(&self,list : *mut WordList){
        r_set_builtin(list);
    }

}
  struct ShiftComand;
  impl CommandExec for ShiftComand{
    fn  excute(&self,list : *mut WordList){
        r_shift_builtin(list);
    }
}    
  struct ShoptComand;
  impl CommandExec for ShoptComand{
    fn  excute(&self,list : *mut WordList){
       unsafe {
        r_shopt_builtin(list);
       }
    }
}
  struct SourceComand;
  impl CommandExec for SourceComand{
    fn  excute(&self,list : *mut WordList){
        r_source_builtin(list);
    }
}
  struct SuspendComand;
  impl CommandExec for SuspendComand{
    fn  excute(&self,list : *mut WordList){
        r_suspend_builtin(list);
    }
}
  struct TestComand;
  impl CommandExec for TestComand{
    fn  excute(&self,list : *mut WordList){
        r_test_builtin(list);
    }
}
  struct TimesComand;
  impl CommandExec for TimesComand{
    fn  excute(&self,list : *mut WordList){
        r_times_builtin(list);
    }
}
  struct TrapComand;
  impl CommandExec for TrapComand{
    fn  excute(&self,list : *mut WordList){
        r_trap_builtin(list);
    }
}
  struct TypeComand;
  impl CommandExec for TypeComand{
    fn  excute(&self,list : *mut WordList){
        unsafe {
            r_type_builtin(list);
        }
       
    }

}
  struct UlimitComand;
  impl CommandExec for UlimitComand{
    fn  excute(&self,list : *mut WordList){
        unsafe {
            r_ulimit_builtin(list);
        }
    }

}
  struct UmaskComand;
  impl CommandExec for UmaskComand{
    fn  excute(&self,list : *mut WordList){
        r_umask_builtin(list);
    }
}
  struct WaitComand;
  impl CommandExec for WaitComand{
     fn  excute(&self,list : *mut WordList){
        r_wait_builtin(list);
    }
}

// 定义接口
pub trait CommandExec {
    fn excute(&self,list : *mut WordList);
}


// 工厂模式
trait Factory {
    fn make_product(&self, product_type : CMDType) ->Box<dyn CommandExec>;
}

struct SimpleFactory;
impl SimpleFactory {
    fn new() -> Self {
        Self
    }
}

impl Factory for SimpleFactory {
    fn make_product(&self, cmd_type : CMDType) -> Box<dyn CommandExec> {
        match cmd_type {
            CMDType::AliasCmd => Box::new(
                AliasComand{}
            ) ,
            CMDType::BindCmd => Box::new(
                BindComand{}
            ),
            CMDType::BreakCmd => Box::new(
                BreakComand{}
            ) ,
            CMDType::BuiltinCmd  => Box::new(
                BuiltinComand{}
            ),
            CMDType::CallerCmd  => Box::new(
                CallerComand{}
            ),
            CMDType::CdCmd  => Box::new(
                CdComand{}
            ),
            CMDType::ColonCmd  => Box::new(
                ColonComand{}
            ),
            CMDType::CommandCmd => Box::new(
                CommandComand{}
            ),
            CMDType::CommonCmd => Box::new(
                CommandComand{}
            ),
            CMDType::CompleteCmd => Box::new(
                CompleteComand{}
            ),
            CMDType::DeclareCmd => Box::new(
                DeclareComand{}
            ),
            CMDType::EchoCmd => Box::new(
                EchoComand{}
            ),
            CMDType::EnableCmd => Box::new(
                EnableComand{}
            ),
            CMDType::EvalCmd => Box::new(
                EvalComand{}
            ),
            CMDType::ExecCmd => Box::new(
                ExecComand{}
            ),
            CMDType::ExitCmd => Box::new(
                ExitComand{}
            ),
            CMDType::FcCmd  => Box::new(
                FcComand{}
            ),
            CMDType::FgbgCmd  => Box::new(
                FgbgComand{}
            ),
            CMDType::GetoptsCmd  => Box::new(
                GetoptsComand{}
            ),
            CMDType::HashCmd  => Box::new(
                HashComand{}
            ),
            CMDType::HelpCmd => Box::new(
                HelpComand{}
            ),
            CMDType::HistoryCmd => Box::new(
                HistoryComand{}
            ),
            CMDType::JobsCmd => Box::new(
                JobsComand{}
            ),
            CMDType::KillCmd => Box::new(
                KillComand{}
            ),
            CMDType::LetCmd => Box::new(
                LetComand{}
            ),
            CMDType::MapfileCmd => Box::new(
                MapfileComand{}
            ),
            CMDType::PrintfCmd => Box::new(
                PrintfComand{}
            ),
            CMDType::PushdCmd => Box::new(
                PushdComand{}
            ),
            CMDType::ReadCmd => Box::new(
                ReadComand{}
            ),
            CMDType::ReservedCmd => Box::new(
                ReservedComand{}
            ),
            CMDType::ReturnCmd => Box::new(
                ReadComand{}
            ),
            CMDType::SetattrCmd => Box::new(
                SetattrComand{}
            ),
            CMDType::SetCmd => Box::new(
                SetComand{}
            ),
            CMDType::ShiftCmd => Box::new(
                ShiftComand{}
            ),
            CMDType::ShoptCmd => Box::new(
                ShoptComand{}
            ),
            CMDType::SourceCmd => Box::new(
                SourceComand{}
            ),
            CMDType::SuspendCmd => Box::new(
                SuspendComand{}
            ),
            CMDType::TestCmd => Box::new(
                TestComand{}
            ),
            CMDType::TimesCmd => Box::new(
                TimesComand{}
            ),
            CMDType::TrapCmd  => Box::new(
                TrapComand{}
            ),
            CMDType::TypeCmd => Box::new(
                TypeComand{}
            ),
            CMDType::UlimitCmd => Box::new(
                UlimitComand{}
            ),
            CMDType::UmaskCmd => Box::new(
                UmaskComand{}
            ),
            CMDType::WaitCmd  => Box::new(
                WaitComand{}
            )
        }
    }
}

fn get_cmd_type (command : *mut i8) -> CMDType{
    let mut types = CMDType::HelpCmd;
    if command == b"alias\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::AliasCmd;
    }
    else if command == b"bind\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::BindCmd;
    }
    else if command == b"break\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::BreakCmd;
    }
    else if command == b"builtin\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::BuiltinCmd;
    }
    else if command == b"caller\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::CallerCmd;
    }
    else if command == b"cd\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::CdCmd;
    }
    else if command == b"colon\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ColonCmd;
    }
    else if command == b"command\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::CommandCmd;
    }
    else if command == b"common\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::CommonCmd;
    }
    else if command == b"complete\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::CompleteCmd;
    }
    else if command == b"declare\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::DeclareCmd;
    }
    else if command == b"echo\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::EchoCmd;
    }
    else if command == b"enable\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::EnableCmd;
    }
    else if command == b"eval\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::EvalCmd;
    }
    else if command == b"exec\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ExecCmd;
    }
    else if command == b"exit\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ExitCmd;
    }
    else if command == b"fc\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::FcCmd;
    }
    else if command == b"fg_bg\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::FgbgCmd;
    }
    else if command == b"getopts\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::GetoptsCmd;
    }
    else if command == b"hash\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::HashCmd;
    }
    else if command == b"help\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::HelpCmd;
    }
    else if command == b"history\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::HistoryCmd;
    }
     else if command == b"jobs\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::JobsCmd;
    }
     else if command == b"kill\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::KillCmd;
    } 
    else if command == b"mapfile\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::MapfileCmd;
    }
     else if command == b"printf\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::PrintfCmd;
    }
     else if command == b"pushd\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::PushdCmd;
    }
     else if command == b"read\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ReadCmd;
    } 
    else if command == b"let\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::LetCmd;
    }
     else if command == b"return\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ReturnCmd;
    
    } 
     else if command == b"set\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::SetCmd;
    }
     else if command == b"setattr\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::SetattrCmd;
    } 
    else if command == b"shift\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ShiftCmd;
    }
     else if command == b"shopt\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::ShoptCmd;
    }
     else if command == b"source\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::SourceCmd;
    }
     else if command == b"suspend\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::SuspendCmd;
    }
     else if command == b"test\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::TestCmd;
    }
     else if command == b"times\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::TimesCmd;
    }
     else if command == b"trap\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::TrapCmd;
    }
     else if command == b"type\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::TypeCmd;
    }
     else if command == b"ulimit\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::UlimitCmd;
    }
     else if command == b"umask\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::UmaskCmd;
    } 
    else if command == b"wait\0" as *const u8 as *const i8 as *mut i8{
        types = CMDType::WaitCmd;
    } 
    return types
}

#[no_mangle]
pub extern "C" fn r_exec_cmd(command : *mut i8, mut list :*mut WordList)  {

    let commandType = get_cmd_type(command);
    let  factory = SimpleFactory::new();
    let cmdCall = factory.make_product(commandType);
    cmdCall.excute(list);
}