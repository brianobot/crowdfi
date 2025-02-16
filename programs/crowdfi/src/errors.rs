use anchor_lang::error_code;


#[error_code]
pub enum CrowdfiError {
    #[msg("Campaing Title is Too Long")]
    CAMPAIGNTITLETOOLONG,
    #[msg("Campaing Description is Too Long")]
    CAMPAIGNDESCRTOOLONG,
    #[msg("Campaing URL is Too Long")]
    CAMPAIGNURLTOOLONG,
    #[msg("Campaign Amount Exceeds allowed Amount for Config")]
    CAMPAIGNMAXAMOUNTEXCEEDED,
    #[msg("Campaign Duration Exceeds allowed Duration for Config")]
    CAMPAIGNDURATIONTOOLONG,
}