use std::sync::Arc;

use crate::FunctionCallError;
use crate::ToolName;
use crate::ToolOutput;
use crate::ToolSpec;

/// Shared runtime contract for model-visible tools.
///
/// Implementations keep the model-visible spec tied to the executable runtime.
/// Host crates can layer routing, hooks, telemetry, or other orchestration on
/// top without reopening the spec/runtime split.
#[async_trait::async_trait]
pub trait ToolExecutor<Invocation>: Send + Sync
where
    Invocation: Send + Sync + 'static,
{
    type Output: ToolOutput + 'static;

    /// The concrete tool name handled by this runtime instance.
    fn tool_name(&self) -> ToolName;

    fn spec(&self) -> Option<ToolSpec> {
        None
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        false
    }

    /// Returns `true` if the invocation might mutate the user's environment.
    ///
    /// Implementations should remain defensive and return `true` whenever the
    /// exact effect of an invocation is uncertain.
    async fn is_mutating(&self, _invocation: &Invocation) -> bool {
        false
    }

    async fn handle(&self, invocation: Invocation) -> Result<Self::Output, FunctionCallError>;
}

#[async_trait::async_trait]
impl<Invocation, T> ToolExecutor<Invocation> for Arc<T>
where
    Invocation: Send + Sync + 'static,
    T: ToolExecutor<Invocation> + ?Sized,
{
    type Output = T::Output;

    fn tool_name(&self) -> ToolName {
        (**self).tool_name()
    }

    fn spec(&self) -> Option<ToolSpec> {
        (**self).spec()
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        (**self).supports_parallel_tool_calls()
    }

    async fn is_mutating(&self, invocation: &Invocation) -> bool {
        (**self).is_mutating(invocation).await
    }

    async fn handle(&self, invocation: Invocation) -> Result<Self::Output, FunctionCallError> {
        (**self).handle(invocation).await
    }
}
