use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
pub enum RoutesEnum {
    Static_UnAuth_Map,
    Static_UnAuth_AuthStatus,
    Static_UnAuth_RpcDashboard,
    Static_UnAuth_RpcApi,
    Static_UnAuth_Notification,
    Static_UnAuth_EmailConfirm,
    Static_UnAuth_ResetPassword,
    Static_UnAuth_ResendConfirmationEmail,
    Static_UnAuth_NewPassword,
    Static_UnAuth_Root,
    Static_UnAuth_Error,
    Static_UnAuth_RegisterApi,
    Static_UnAuth_Register,
    Static_UnAuth_Register_Wallet,
    Static_UnAuth_Version,
    Static_UnAuth_Unsubscribe,
    Static_UnAuth_Login,
    Static_UnAuth_Login_Wallet,
    Static_UnAuth_HealthCheck,
    Static_UnAuth_Health,
    Static_Auth_Twitter_Login,
    Static_Auth_Edit_Invite,
    Static_Auth_Call_To_Action,
    Static_Auth_Logout,
    Static_Auth_Dashboard,
    Static_Auth_Daily_Leaderboard,
    Static_UnAuth_Twitter_Callback,
    Api_ConnectWallet,
    Api_ReportUptime,
    Api_SubmitBandwidth,
    Api_GetToken,
    Api_GetTask,
    Api_SubmitTask,
    Api_GetStats,
    Api_GetLatestInviteCode,
    Api_CreateTaskWithToken,
    Api_CheckToken,
    Api_EMailViaToken,
    Api_Dashboard,
    Api_ReportsQueue,
}

impl Display for RoutesEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            RoutesEnum::Static_UnAuth_Version => write!(f, "/version"),
            RoutesEnum::Static_Auth_Twitter_Login => write!(f, "/twitter/login"),
            RoutesEnum::Static_UnAuth_Map => write!(f, "/map"),
            RoutesEnum::Static_UnAuth_Twitter_Callback => write!(f, "/twitter/callback"),
            RoutesEnum::Static_UnAuth_AuthStatus => write!(f, "/auth_status"),
            RoutesEnum::Static_UnAuth_RpcDashboard => write!(f, "/rpc_dashboard"),
            RoutesEnum::Static_UnAuth_RpcApi => write!(f, "/rpc_api"),
            RoutesEnum::Static_UnAuth_Notification => write!(f, "/notification"),
            RoutesEnum::Static_Auth_Daily_Leaderboard => write!(f, "/daily_leaderboard"),
            RoutesEnum::Static_UnAuth_EmailConfirm => write!(f, "/email_confirm"),
            RoutesEnum::Static_UnAuth_ResetPassword => write!(f, "/reset_password"),
            RoutesEnum::Static_UnAuth_ResendConfirmationEmail => {
                write!(f, "/resend_confirmation_email")
            }
            RoutesEnum::Static_UnAuth_NewPassword => write!(f, "/new_password"),
            RoutesEnum::Static_UnAuth_Root => write!(f, "/"),
            RoutesEnum::Static_UnAuth_Error => write!(f, "/error"),
            RoutesEnum::Static_UnAuth_RegisterApi => write!(f, "/register_api"),
            RoutesEnum::Static_UnAuth_Register => write!(f, "/register"),
            RoutesEnum::Static_UnAuth_Register_Wallet => write!(f, "/register_wallet"),
            RoutesEnum::Static_UnAuth_Login => write!(f, "/login"),
            RoutesEnum::Static_UnAuth_Login_Wallet => write!(f, "/login_wallet"),
            RoutesEnum::Static_UnAuth_HealthCheck => write!(f, "/health_check"),
            RoutesEnum::Static_UnAuth_Health => write!(f, "/health"),
            RoutesEnum::Static_Auth_Logout => write!(f, "/logout"),
            RoutesEnum::Static_Auth_Dashboard => write!(f, "/dashboard"),
            RoutesEnum::Static_Auth_Edit_Invite => write!(f, "/edit_invite_code"),
            RoutesEnum::Static_Auth_Call_To_Action => write!(f, "/call_to_action"),
            RoutesEnum::Api_ConnectWallet => write!(f, "/connect_wallet"),
            RoutesEnum::Api_ReportUptime => write!(f, "/report_uptime"),
            RoutesEnum::Api_SubmitBandwidth => write!(f, "/submit_bandwidth"),
            RoutesEnum::Api_GetToken => write!(f, "/get_token"),
            RoutesEnum::Api_GetTask => write!(f, "/get_task"),
            RoutesEnum::Api_SubmitTask => write!(f, "/submit_task"),
            RoutesEnum::Api_GetStats => write!(f, "/get_stats"),
            RoutesEnum::Api_GetLatestInviteCode => write!(f, "/get_latest_invite_code"),
            RoutesEnum::Api_CreateTaskWithToken => write!(f, "/create_task_with_token"),
            RoutesEnum::Api_CheckToken => write!(f, "/check_token"),
            RoutesEnum::Api_EMailViaToken => write!(f, "/get_email_via_token"),
            RoutesEnum::Api_Dashboard => write!(f, "/dashboard"),
            RoutesEnum::Api_ReportsQueue => write!(f, "/admin/reports_queue"),
            RoutesEnum::Static_UnAuth_Unsubscribe => write!(f, "/unsubscribe"),
        }
    }
}
