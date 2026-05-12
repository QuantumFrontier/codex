use codex_tools::JsonToolOutput;
use codex_tools::ToolCall;
use codex_tools::ToolExecutor;

/// Model-facing output returned by extension-owned tools.
pub type ExtensionToolOutput = JsonToolOutput;

/// Extension-facing executable tool object pinned to the extension invocation
/// and output types.
pub type ExtensionToolExecutor = dyn ToolExecutor<ToolCall, Output = ExtensionToolOutput>;
