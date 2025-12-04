use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    BuiltInToolName,
    BuiltInToolTrait,
    ToolExecutionError,
    ToolExecutionOutput,
    ToolExecutionOutputItem,
    ToolExecutionResult,
};

const EXECUTE_CMD_TOOL_DESCRIPTION: &str = r#"
A tool for executing bash commands.

WHEN TO USE THIS TOOL:
- Use only as a last-resort when no other available tool can accomplish the task

HOW TO USE:
- Provide the command to execute

FEATURES:

LIMITATIONS:
- Does not respect user's bash profile or aliases

TIPS:
- Use the fileRead and fileWrite tools for reading and modifying files
"#;

const EXECUTE_CMD_SCHEMA: &str = r#"
{
    "type": "object",
    "properties": {
        "command": {
            "type": "string",
            "description": "Command to execute"
        }
    },
    "required": [
        "command"
    ]
}
"#;

impl BuiltInToolTrait for ExecuteCmd {
    fn name() -> BuiltInToolName {
        BuiltInToolName::ExecuteCmd
    }

    fn description() -> std::borrow::Cow<'static, str> {
        EXECUTE_CMD_TOOL_DESCRIPTION.into()
    }

    fn input_schema() -> std::borrow::Cow<'static, str> {
        EXECUTE_CMD_SCHEMA.into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteCmd {
    pub command: String,
}

impl ExecuteCmd {
    pub async fn validate(&self) -> Result<(), String> {
        if self.command.is_empty() {
            Err("Command must not be empty".to_string())
        } else {
            Ok(())
        }
    }

    #[cfg(target_family = "unix")]
    pub async fn execute(&self) -> ToolExecutionResult {
        unix::execute_command(self).await
    }

    #[cfg(not(target_family = "unix"))]
    pub async fn execute(&self) -> ToolExecutionResult {
        Err(ToolExecutionError::Custom("The 'executeCmd' tool is not supported on this operating system.".to_string()))
    }
}


#[cfg(target_family = "unix")]
mod unix {
    use std::collections::HashMap;
    use std::process::Stdio;

    use bstr::ByteSlice as _;
    use tokio::process::Command;

    use super::{ExecuteCmd, ToolExecutionError, ToolExecutionOutput, ToolExecutionOutputItem, ToolExecutionResult};
    use crate::agent::util::consts::{
        USER_AGENT_APP_NAME,
        USER_AGENT_ENV_VAR,
        USER_AGENT_VERSION_KEY,
        USER_AGENT_VERSION_VALUE,
    };

    pub(super) async fn execute_command(cmd: &ExecuteCmd) -> ToolExecutionResult {
        let shell = std::env::var("AMAZON_Q_CHAT_SHELL").unwrap_or("bash".to_string());

        let env_vars = env_vars_with_user_agent();

        let child = Command::new(shell)
            .arg("-c")
            .arg(&cmd.command)
            .envs(env_vars)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ToolExecutionError::io(format!("Failed to spawn command '{}'", &cmd.command), e))?;

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| ToolExecutionError::io(format!("No exit status for '{}'", &cmd.command), e))?;

        let exit_status = output.status;
        let clean_stdout = sanitize_unicode_tags(output.stdout.to_str_lossy());
        let clean_stderr = sanitize_unicode_tags(output.stderr.to_str_lossy());

        let result = serde_json::json!({
            "exit_status": exit_status.to_string(),
            "stdout": clean_stdout,
            "stderr": clean_stderr,
        });

        Ok(ToolExecutionOutput {
            items: vec![ToolExecutionOutputItem::Json(result)],
        })
    }

    fn is_hidden(c: char) -> bool {
        match c {
            '\u{E0000}'..='\u{E007F}' |     // TAG characters (used for hidden prompts)
            '\u{200B}'..='\u{200F}'  |      // zero-width space, ZWJ, ZWNJ, RTL/LTR marks
            '\u{2028}'..='\u{202F}'  |      // line / paragraph separators, narrow NB-SP
            '\u{205F}'..='\u{206F}'  |      // format control characters
            '\u{FFF0}'..='\u{FFFC}'  |
            '\u{FFFE}'..='\u{FFFF}'   // Specials block (non-characters)
            => true,
            _ => false,
        }
    }

    fn sanitize_unicode_tags(text: impl AsRef<str>) -> String {
        let mut removed = 0;
        let out: String = text
            .as_ref()
            .chars()
            .filter(|&c| {
                let bad = is_hidden(c);
                if bad {
                    removed += 1;
                }
                !bad
            })
            .collect();

        if removed > 0 {
            tracing::debug!("Detected and removed {} hidden chars", removed);
        }
        out
    }

    fn env_vars_with_user_agent() -> HashMap<String, String> {
        let mut env_vars: HashMap<String, String> = std::env::vars().collect();

        let user_agent_metadata_value = format!(
            "{} {}/{}",
            USER_AGENT_APP_NAME, USER_AGENT_VERSION_KEY, USER_AGENT_VERSION_VALUE
        );

        let existing_value = std::env::var(USER_AGENT_ENV_VAR).ok();

        if let Some(existing_value) = existing_value {
            if !existing_value.is_empty() {
                env_vars.insert(
                    USER_AGENT_ENV_VAR.to_string(),
                    format!("{} {}", existing_value, user_agent_metadata_value),
                );
            } else {
                env_vars.insert(USER_AGENT_ENV_VAR.to_string(), user_agent_metadata_value);
            }
        } else {
            env_vars.insert(USER_AGENT_ENV_VAR.to_string(), user_agent_metadata_value);
        }

        env_vars
    }
}