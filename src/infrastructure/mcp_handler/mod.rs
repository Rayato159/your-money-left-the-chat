use std::sync::Arc;

use crate::domain::value_objects::spending_scanner::{
    AddMonthlySpendingModel, SpendingScannerFilter,
};
use crate::domain::value_objects::tax_simulator::{
    AddTaxDeductionsListModel, RemoveTaxDeductionsListModel, TaxSimulateRequestModel,
};
use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::RequestContext;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, model::*, prompt, prompt_handler,
    prompt_router, tool, tool_handler, tool_router,
};
use serde_json::json;

use crate::{
    application::use_cases::{
        cash_flow::CashFlowUseCase, spending_scanner::SpendingScannerUseCase,
        tax_simulator::TaxSimulatorUseCase,
    },
    domain::value_objects::{
        cash_flow::{RecordCashFlowModel, RecordCashFlowWithDateModel},
        spending_scanner::RemoveMonthlySpendingModel,
    },
};

#[derive(Clone)]
pub struct MCPHandler {
    cash_flow_use_case: Arc<CashFlowUseCase>,
    spending_scanner_use_case: Arc<SpendingScannerUseCase>,
    tax_simulator_use_case: Arc<TaxSimulatorUseCase>,
    tool_router: ToolRouter<MCPHandler>,
    prompt_router: PromptRouter<MCPHandler>,
}

#[tool_router]
impl MCPHandler {
    pub fn new(
        cash_flow_use_case: Arc<CashFlowUseCase>,
        spending_scanner_use_case: Arc<SpendingScannerUseCase>,
        tax_simulator_use_case: Arc<TaxSimulatorUseCase>,
    ) -> Self {
        Self {
            cash_flow_use_case,
            spending_scanner_use_case,
            tax_simulator_use_case,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Record a cash flow ledger transaction")]
    pub async fn record_cash_flow(
        &self,
        Parameters(record_cash_flow_model): Parameters<RecordCashFlowModel>,
    ) -> Result<CallToolResult, McpError> {
        match self.cash_flow_use_case.record(record_cash_flow_model).await {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Cash flow ledger transaction recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record a cash flow ledger transaction with date")]
    pub async fn record_cash_flow_with_date(
        &self,
        Parameters(record_cash_flow_with_date_model): Parameters<RecordCashFlowWithDateModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .cash_flow_use_case
            .record_with_date(record_cash_flow_with_date_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Cash flow ledger transaction recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(
        description = "See how much money you have spent by date range: today, this month, this year, or lifetime."
    )]
    pub async fn spending_scanner(
        &self,
        Parameters(spending_scanner_filter): Parameters<SpendingScannerFilter>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .scan(spending_scanner_filter)
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(
        description = "Visualize your spending by category: today, this month, this year, or lifetime."
    )]
    pub async fn spending_visualizer(
        &self,
        Parameters(spending_scanner_filter): Parameters<SpendingScannerFilter>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .visualize(spending_scanner_filter)
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "View all monthly spending list")]
    pub async fn view_all_monthly_spending_list(&self) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .view_all_monthly_spending_list()
            .await
        {
            Ok(result) => {
                if let Ok(res_json) = Content::json(result) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Add monthly spending into the list (due_date format: DD)")]
    pub async fn add_monthly_spending(
        &self,
        Parameters(add_monthly_spending_model): Parameters<AddMonthlySpendingModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .add_monthly_spending(add_monthly_spending_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "New monthly spending list added successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Remove monthly spending from the list")]
    pub async fn remove_monthly_spending(
        &self,
        Parameters(remove_monthly_spending_model): Parameters<RemoveMonthlySpendingModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .remove_monthly_spending(remove_monthly_spending_model)
            .await
        {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(
                "Remove monthly spending from the list successfully",
            )])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "View all tax deductions list")]
    pub async fn view_all_tax_deductions_list(&self) -> Result<CallToolResult, McpError> {
        match self
            .tax_simulator_use_case
            .view_all_tax_deductions_list()
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Add tax deduction list")]
    pub async fn add_tax_deduction_list(
        &self,
        Parameters(add_tax_deduction_list_model): Parameters<AddTaxDeductionsListModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .tax_simulator_use_case
            .add_tax_deduction_list(add_tax_deduction_list_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Add tax deduction list successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Remove tax deduction list")]
    pub async fn remove_tax_deduction_list(
        &self,
        Parameters(remove_tax_deduction_list_model): Parameters<RemoveTaxDeductionsListModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .tax_simulator_use_case
            .remove_tax_deduction_list(remove_tax_deduction_list_model)
            .await
        {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(
                "Remove tax deduction list successfully: id:",
            )])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Calculate, simulate tax for a given year")]
    pub async fn simulate_tax(
        &self,
        Parameters(tax_simulate_request_model): Parameters<TaxSimulateRequestModel>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .tax_simulator_use_case
            .simulate(tax_simulate_request_model)
            .await
        {
            Ok(result) => {
                if let Ok(res_json) = Content::json(result) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

#[prompt_router]
impl MCPHandler {
    /// This is an example prompt that takes one required argument, message
    #[prompt(name = "example_prompt")]
    async fn example_prompt(
        &self,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        let prompt = "Hello! This is an example prompt from MCPHandler. You can customize this prompt to fit your needs.".to_string();
        Ok(vec![PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(prompt),
        }])
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for MCPHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
                protocol_version: ProtocolVersion::V_2024_11_05,
                capabilities: ServerCapabilities::builder()
                    .enable_prompts()
                    .enable_resources()
                    .enable_tools()
                    .build(),
                server_info: Implementation::from_build_env(),
                instructions: Some("This server provides counter tools and prompts. Tools: increment, decrement, get_value, say_hello, echo, sum. Prompts: example_prompt (takes a message), counter_analysis (analyzes counter state with a goal).".to_string()),
            }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}
