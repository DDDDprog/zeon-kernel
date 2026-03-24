/*
 *  ███████╗███████╗ ██████╗ ███╗   ██╗
 *  ╚══███╔╝██╔════╝██╔═══██╗████╗  ██║
 *    ███╔╝ █████╗  ██║   ██║██╔██╗ ██║
 *   ███╔╝  ██╔══╝  ██║   ██║██║╚██╗██║
 *  ███████╗███████╗╚██████╔╝██║ ╚████║
 *  ╚══════╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝
 *
 * Zeon - Pure Rust Operating System
 * https://github.com/DDDDprog/zeon-kernel
 */

// Zeon - Pure Rust Operating System
// https://github.com/DDDDprog/zeon-kernel

pub async fn sys_nanosleep(rqtp: TUA<TimeSpec>, rmtp: TUA<TimeSpec>) -> Result<usize> {
    let timespec: Duration = TimeSpec::copy_from_user(rqtp).await?.into();
    let started_at = now().unwrap();

    match sleep(timespec).interruptable().await {
        InterruptResult::Interrupted => {
            if !rmtp.is_null() {
                let elapsed = now().unwrap() - started_at;
                copy_to_user(rmtp, (timespec - elapsed).into()).await?;
            }
            Err(KernelError::Interrupted)
        }
        InterruptResult::Uninterrupted(()) => Ok(0),
    }
}

pub async fn sys_clock_nanosleep(
    _clock_id: i32,
    _flags: u32,
    rqtp: TUA<TimeSpec>,
    rmtp: TUA<TimeSpec>,
) -> Result<usize> {
    sys_nanosleep(rqtp, rmtp).await
}
