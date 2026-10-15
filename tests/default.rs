use super::*;

#[test]
fn default_attribute_overrides_first_recipe() {
  Test::new()
    .justfile(
      "
        foo:
          @echo FOO

        [default]
        bar:
          @echo BAR
      ",
    )
    .stdout("BAR\n")
    .success();
}

#[test]
fn default_attribute_may_only_appear_once_per_justfile() {
  Test::new()
    .justfile(
      "
        [default]
        foo:

        [default]
        bar:
      ",
    )
    .stderr(
      "
        error: recipe `bar` has duplicate `[default]` attribute, which may only appear once per module
         ——▶ justfile:5:1
          │
        5 │ bar:
          │ ^^^
      "
    )
    .failure();
}

#[test]
fn duplicate_default_attribute_blames_source_later_recipe() {
  Test::new()
    .justfile(
      "
        [default]
        bar:

        [default]
        foo:
      ",
    )
    .arg("--summary")
    .stderr(
      "
        error: recipe `foo` has duplicate `[default]` attribute, which may only appear once per module
         ——▶ justfile:5:1
          │
        5 │ foo:
          │ ^^^
      ",
    )
    .failure();
}

#[test]
fn default_list_lists_recipes() {
  Test::new()
    .justfile(
      "
        set default-list := true

        foo:
          @echo foo

        bar:
      ",
    )
    .stdout(
      "
        Available recipes:
            bar
            foo
      ",
    )
    .success();
}

#[test]
fn default_list_presentation() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-heading := \"Cool stuff…\\n\"
        set default-list-prefix := \"..........\"
        set default-list-unsorted

        foo:

        bar:
      ",
    )
    .stdout(
      "
        Cool stuff…
        ..........foo
        ..........bar
      ",
    )
    .success();
}

#[test]
fn default_list_alias_style() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-alias-style := 'separate'

        alias f := foo

        # comment
        foo:
      ",
    )
    .stdout(
      "
        Available recipes:
            foo # comment
            f   # alias for `foo`
      ",
    )
    .success();
}

#[test]
fn default_list_no_aliases() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-no-aliases

        alias f := foo

        foo:
      ",
    )
    .stdout(
      "
        Available recipes:
            foo
      ",
    )
    .success();
}

#[test]
fn default_list_groups() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-groups := ['build']

        [group('test')]
        test:

        [group('build')]
        build:
      ",
    )
    .stdout(
      "
        Available recipes:
            [build]
            build
      ",
    )
    .success();
}

#[test]
fn default_list_submodules() {
  Test::new()
    .write("foo.just", "bar:")
    .justfile(
      "
        set default-list
        set default-list-submodules

        mod foo

        root:
      ",
    )
    .stdout(
      "
        Available recipes:
            root
            foo:
                bar
      ",
    )
    .success();
}

#[test]
fn default_list_command_line_options_override_settings() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-heading := \"Setting\\n\"
        set default-list-prefix := \"S \"

        b:

        a:
      ",
    )
    .args([
      "--list-heading",
      "Command line\n",
      "--list-prefix",
      "C ",
      "--unsorted",
    ])
    .stdout(
      "
        Command line
        C b
        C a
      ",
    )
    .success();
}

#[test]
fn default_list_environment_options_override_settings() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-alias-style := 'left'
        set default-list-heading := \"Setting\\n\"

        alias f := foo

        # comment
        foo:
      ",
    )
    .env("JUST_ALIAS_STYLE", "separate")
    .env("JUST_LIST_HEADING", "Environment\n")
    .stdout(
      "
        Environment
            foo # comment
            f   # alias for `foo`
      ",
    )
    .success();
}

#[test]
fn default_list_false_environment_options_override_settings() {
  Test::new()
    .justfile(
      "
        set default-list
        set default-list-no-aliases
        set default-list-unsorted

        alias z := b

        b:

        a:
      ",
    )
    .env("JUST_NO_ALIASES", "false")
    .env("JUST_UNSORTED", "false")
    .stdout(
      "
        Available recipes:
            a
            b # [alias: z]
      ",
    )
    .success();
}

#[test]
fn default_list_settings_do_not_affect_explicit_list() {
  Test::new()
    .justfile(
      "
        set default-list-heading := \"Setting\\n\"
        set default-list-prefix := \"S \"
        set default-list-unsorted

        b:

        a:
      ",
    )
    .arg("--list")
    .stdout(
      "
        Available recipes:
            a
            b
      ",
    )
    .success();
}

#[test]
fn default_list_uses_target_module_settings_without_inheritance() {
  Test::new()
    .write(
      "foo.just",
      "
        set default-list
        set default-list-prefix := 'child '

        bar:
      ",
    )
    .justfile(
      "
        set default-list-heading := \"Root\\n\"
        set default-list-prefix := 'root '

        mod foo
      ",
    )
    .arg("foo")
    .stdout(
      "
        Available recipes:
        child bar
      ",
    )
    .success();
}

#[test]
fn default_list_flag_uses_target_module_settings() {
  Test::new()
    .write(
      "foo.just",
      "
        set default-list-heading := \"Module\\n\"
        set default-list-prefix := 'module '

        bar:
      ",
    )
    .justfile("mod foo")
    .args(["--default-list", "foo"])
    .stdout(
      "
        Module
        module bar
      ",
    )
    .success();
}

#[test]
fn default_list_flag_lists_recipes() {
  Test::new()
    .justfile(
      "
        foo:
          @echo foo

        bar:
      ",
    )
    .arg("--default-list")
    .stdout(
      "
        Available recipes:
            bar
            foo
      ",
    )
    .success();
}

#[test]
fn default_list_flag_does_not_override_explicit_recipe() {
  Test::new()
    .justfile(
      "
        foo:
          @echo foo

        bar:
          @echo bar
      ",
    )
    .arg("--default-list")
    .arg("bar")
    .stdout("bar\n")
    .success();
}

#[test]
fn default_list_false_runs_default_recipe() {
  Test::new()
    .justfile(
      "
        set default-list := false

        foo:
          @echo foo

        bar:
      ",
    )
    .stdout("foo\n")
    .success();
}

#[test]
fn default_list_does_not_override_explicit_recipe() {
  Test::new()
    .justfile(
      "
        set default-list

        foo:
          @echo foo

        bar:
          @echo bar
      ",
    )
    .arg("bar")
    .stdout("bar\n")
    .success();
}

#[test]
fn default_list_allows_default_recipe_with_arguments() {
  Test::new()
    .justfile(
      "
        set default-list

        foo bar:
          @echo {{bar}}

        baz:
      ",
    )
    .stdout(
      "
        Available recipes:
            baz
            foo bar
      ",
    )
    .success();
}
