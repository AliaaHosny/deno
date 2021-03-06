// Copyright 2018-2019 the Deno authors. All rights reserved. MIT license.
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use deno::v8_set_flags;

// Creates vector of strings, Vec<String>
#[cfg(test)]
macro_rules! svec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DenoFlags {
  pub log_debug: bool,
  pub version: bool,
  pub reload: bool,
  pub allow_read: bool,
  pub allow_write: bool,
  pub allow_net: bool,
  pub allow_env: bool,
  pub allow_run: bool,
  pub allow_high_precision: bool,
  pub no_prompts: bool,
  pub types: bool,
  pub prefetch: bool,
  pub info: bool,
  pub fmt: bool,
  pub eval: bool,
}

impl<'a> From<ArgMatches<'a>> for DenoFlags {
  fn from(matches: ArgMatches) -> DenoFlags {
    let mut flags = DenoFlags::default();

    if matches.is_present("log-debug") {
      flags.log_debug = true;
    }
    if matches.is_present("version") {
      flags.version = true;
    }
    if matches.is_present("reload") {
      flags.reload = true;
    }
    if matches.is_present("allow-read") {
      flags.allow_read = true;
    }
    if matches.is_present("allow-write") {
      flags.allow_write = true;
    }
    if matches.is_present("allow-net") {
      flags.allow_net = true;
    }
    if matches.is_present("allow-env") {
      flags.allow_env = true;
    }
    if matches.is_present("allow-run") {
      flags.allow_run = true;
    }
    if matches.is_present("allow-high-precision") {
      flags.allow_high_precision = true;
    }
    if matches.is_present("allow-all") {
      flags.allow_read = true;
      flags.allow_env = true;
      flags.allow_net = true;
      flags.allow_run = true;
      flags.allow_read = true;
      flags.allow_write = true;
      flags.allow_high_precision = true;
    }
    if matches.is_present("no-prompt") {
      flags.no_prompts = true;
    }
    if matches.is_present("types") {
      flags.types = true;
    }
    if matches.is_present("prefetch") {
      flags.prefetch = true;
    }
    if matches.is_present("info") {
      flags.info = true;
    }
    if matches.is_present("fmt") {
      flags.fmt = true;
    }
    if matches.is_present("eval") {
      flags.eval = true;
    }

    flags
  }
}

static ENV_VARIABLES_HELP: &str = "ENVIRONMENT VARIABLES:
    DENO_DIR        Set deno's base directory
    NO_COLOR        Set to disable color";

fn create_cli_app<'a, 'b>() -> App<'a, 'b> {
  let cli_app = App::new("deno")
    .bin_name("deno")
    .global_settings(&[AppSettings::ColorNever])
    .settings(&[
      AppSettings::AllowExternalSubcommands,
      AppSettings::DisableHelpSubcommand,
    ]).after_help(ENV_VARIABLES_HELP)
    .arg(
      Arg::with_name("version")
        .short("v")
        .long("version")
        .help("Print the version"),
    ).arg(
      Arg::with_name("allow-read")
        .long("allow-read")
        .help("Allow file system read access"),
    ).arg(
      Arg::with_name("allow-write")
        .long("allow-write")
        .help("Allow file system write access"),
    ).arg(
      Arg::with_name("allow-net")
        .long("allow-net")
        .help("Allow network access"),
    ).arg(
      Arg::with_name("allow-env")
        .long("allow-env")
        .help("Allow environment access"),
    ).arg(
      Arg::with_name("allow-run")
        .long("allow-run")
        .help("Allow running subprocesses"),
    ).arg(
      Arg::with_name("allow-high-precision")
        .long("allow-high-precision")
        .help("Allow high precision time measurement"),
    ).arg(
      Arg::with_name("allow-all")
        .short("A")
        .long("allow-all")
        .help("Allow all permissions"),
    ).arg(
      Arg::with_name("no-prompt")
        .long("no-prompt")
        .help("Do not use prompts"),
    ).arg(
      Arg::with_name("log-debug")
        .short("D")
        .long("log-debug")
        .help("Log debug output"),
    ).arg(
      Arg::with_name("reload")
        .short("r")
        .long("reload")
        .help("Reload source code cache (recompile TypeScript)"),
    ).arg(
      Arg::with_name("v8-options")
        .long("v8-options")
        .help("Print V8 command line options"),
    ).arg(
      Arg::with_name("v8-flags")
        .long("v8-flags")
        .takes_value(true)
        .require_equals(true)
        .help("Set V8 command line options"),
    ).arg(
      Arg::with_name("types")
        .long("types")
        .help("Print runtime TypeScript declarations"),
    ).arg(
      Arg::with_name("prefetch")
        .long("prefetch")
        .help("Prefetch the dependencies"),
    ).subcommand(
      SubCommand::with_name("info")
        .setting(AppSettings::DisableVersion)
        .about("Show source file related info")
        .arg(Arg::with_name("file").takes_value(true).required(true)),
    ).subcommand(
      SubCommand::with_name("eval")
        .setting(AppSettings::DisableVersion)
        .about("Eval script")
        .arg(Arg::with_name("code").takes_value(true).required(true)),
    ).subcommand(
      SubCommand::with_name("fmt")
        .setting(AppSettings::DisableVersion)
        .about("Format files")
        .arg(
          Arg::with_name("files")
            .takes_value(true)
            .multiple(true)
            .required(true),
        ),
    ).subcommand(
      // this is a fake subcommand - it's used in conjunction with
      // AppSettings:AllowExternalSubcommand to treat it as an
      // entry point script
      SubCommand::with_name("<script>").about("Script to run"),
    );

  cli_app
}

#[cfg_attr(feature = "cargo-clippy", allow(stutter))]
pub fn set_flags(
  args: Vec<String>,
) -> Result<(DenoFlags, Vec<String>), String> {
  let mut rest_argv: Vec<String> = vec!["deno".to_string()];
  let cli_app = create_cli_app();
  let matches = cli_app.get_matches_from(args);

  match matches.subcommand() {
    ("eval", Some(info_match)) => {
      let code: &str = info_match.value_of("code").unwrap();
      rest_argv.extend(vec![code.to_string()]);
    }
    ("info", Some(info_match)) => {
      let file: &str = info_match.value_of("file").unwrap();
      rest_argv.extend(vec![file.to_string()]);
    }
    ("fmt", Some(fmt_match)) => {
      let files: Vec<String> = fmt_match
        .values_of("files")
        .unwrap()
        .map(String::from)
        .collect();
      rest_argv.extend(files);
    }
    (script, Some(script_match)) => {
      rest_argv.extend(vec![script.to_string()]);
      // check if there are any extra arguments that should
      // be passed to script
      if script_match.is_present("") {
        let script_args: Vec<String> = script_match
          .values_of("")
          .unwrap()
          .map(String::from)
          .collect();
        rest_argv.extend(script_args);
      }
    }
    _ => {}
  }

  if matches.is_present("v8-options") {
    // display v8 help and exit
    // TODO(bartlomieju): this relies on `v8_set_flags` to swap `--v8-options` to help
    v8_set_flags(vec!["deno".to_string(), "--v8-options".to_string()]);
  }

  if matches.is_present("v8-flags") {
    let mut v8_flags: Vec<String> = matches
      .values_of("v8-flags")
      .unwrap()
      .map(String::from)
      .collect();

    v8_flags.insert(1, "deno".to_string());
    v8_set_flags(v8_flags);
  }

  let flags = DenoFlags::from(matches);
  Ok((flags, rest_argv))
}

#[test]
fn test_set_flags_1() {
  let (flags, rest) = set_flags(svec!["deno", "--version"]).unwrap();
  assert_eq!(rest, svec!["deno"]);
  assert_eq!(
    flags,
    DenoFlags {
      version: true,
      ..DenoFlags::default()
    }
  );
}

#[test]
fn test_set_flags_2() {
  let (flags, rest) =
    set_flags(svec!["deno", "-r", "-D", "script.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "script.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      log_debug: true,
      reload: true,
      ..DenoFlags::default()
    }
  );
}

#[test]
fn test_set_flags_3() {
  let (flags, rest) =
    set_flags(svec!["deno", "-r", "--allow-write", "script.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "script.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      reload: true,
      allow_write: true,
      ..DenoFlags::default()
    }
  );
}

#[test]
fn test_set_flags_4() {
  let (flags, rest) =
    set_flags(svec!["deno", "-Dr", "--allow-write", "script.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "script.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      log_debug: true,
      reload: true,
      allow_write: true,
      ..DenoFlags::default()
    }
  );
}

#[test]
fn test_set_flags_5() {
  let (flags, rest) = set_flags(svec!["deno", "--types"]).unwrap();
  assert_eq!(rest, svec!["deno"]);
  assert_eq!(
    flags,
    DenoFlags {
      types: true,
      ..DenoFlags::default()
    }
  )
}

#[test]
fn test_set_flags_6() {
  let (flags, rest) =
    set_flags(svec!["deno", "--allow-net", "gist.ts", "--title", "X"]).unwrap();
  assert_eq!(rest, svec!["deno", "gist.ts", "--title", "X"]);
  assert_eq!(
    flags,
    DenoFlags {
      allow_net: true,
      ..DenoFlags::default()
    }
  )
}

#[test]
fn test_set_flags_7() {
  let (flags, rest) =
    set_flags(svec!["deno", "--allow-all", "gist.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "gist.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      allow_net: true,
      allow_env: true,
      allow_run: true,
      allow_read: true,
      allow_write: true,
      allow_high_precision: true,
      ..DenoFlags::default()
    }
  )
}

#[test]
fn test_set_flags_8() {
  let (flags, rest) =
    set_flags(svec!["deno", "--allow-read", "gist.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "gist.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      allow_read: true,
      ..DenoFlags::default()
    }
  )
}

#[test]
fn test_set_flags_9() {
  let (flags, rest) =
    set_flags(svec!["deno", "--allow-high-precision", "script.ts"]).unwrap();
  assert_eq!(rest, svec!["deno", "script.ts"]);
  assert_eq!(
    flags,
    DenoFlags {
      allow_high_precision: true,
      ..DenoFlags::default()
    }
  )
}
