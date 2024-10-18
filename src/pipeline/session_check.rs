use crate::state_services::err_set::ErrState;

use super::processor::Processor;

pub trait SessionCheck {
    fn session_check_http(&mut self) -> Result<&mut Self, ErrState>;
}

impl SessionCheck for Processor {
    fn session_check_http(&mut self) -> Result<&mut Self, ErrState> {
        self.state.stage = 1;

        if let Some(session_vec) = self.state.session_id {
             // セッションIDがある場合
            if let Some(user_ruid) = self.app_set.session.user(session_vec.clone())? {
                // セッションIDからユーザーIDを取得
                self.app_set.session.update_last_access_time(session_vec.clone())?;
                self.app_set.user.update_last_access_time(&user_ruid)?;
                let user_data = self.app_set.user.get(&user_ruid)?;
                self.state.user_ruid = format!("{:x}", user_ruid);
                self.state.user_perm = user_data.perm;
                return Ok(self);
            } else {
                // セッションIDが無効な場合
                let new_session_vec = self.app_set.session.set()?;
            }
        }
    
            // // セッションを生成
            // let new_session_vec = self.app.session.set()?;
            // // ユーザーセッションを生成
            // let guest_user_ruid = self.app.ruid.generate(self.app.config.ruid_prefix.USER_EXAMPLE_ID, None);
            // let everyoune_permission : u128 = (self.app.config.ruid_prefix.USER_EXAMPLE_ID as u128) << 112;
            // self.app.session.add(new_session_vec.clone(), guest_user_ruid.to_u128())?;
            // self.app.user.set(
            //     &guest_user_ruid.to_u128(),
            //     &format!("@guest{}", guest_user_ruid.to_string()),
            //     &0,
            //     &vec![everyoune_permission],
            // )?;
            // // 情報をステートにコピー
            // self.state.user_ruid = format!("{:x}", guest_user_ruid.to_u128());
            // self.state.session_id = Some(self.app.session.vec_to_base64(new_session_vec));
            // let user_data = self.app.user.get(&guest_user_ruid.to_u128())?;
            // self.state.user_perm = user_data.perm.iter().map(|&num| format!("{:x}", num)).collect();
            // return Ok(());


    }
}