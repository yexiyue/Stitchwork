use super::request::ForwardedTools;
use futures::FutureExt;
use rig::tool::ToolDyn;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FrontendTool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl ToolDyn for FrontendTool {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn definition<'a>(
        &'a self,
        _prompt: String,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, rig::completion::ToolDefinition> {
        async move {
            rig::completion::ToolDefinition {
                name: self.name.clone(),
                description: self.description.clone(),
                parameters: self.parameters.clone(),
            }
        }
        .boxed()
    }

    fn call<'a>(
        &'a self,
        _args: String,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, Result<String, rig::tool::ToolError>> {
        // 前端工具不应该在后端执行，返回错误
        // 正常情况下 Hook 会在执行前取消，但如果并发导致执行，返回错误而不是 panic
        async move {
            Err(rig::tool::ToolError::ToolCallError(
                "Frontend tool should not be executed on backend".into(),
            ))
        }
        .boxed()
    }
}

impl From<&ForwardedTools> for Vec<Box<dyn ToolDyn>> {
    fn from(tools: &ForwardedTools) -> Self {
        tools
            .tools
            .iter()
            .map(|(name, tool)| {
                Box::new(FrontendTool {
                    name: name.clone(),
                    description: tool.description.clone(),
                    parameters: tool.parameters.clone(),
                }) as Box<dyn ToolDyn>
            })
            .collect()
    }
}
