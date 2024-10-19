use crate::state_services::err_set::ErrState;

use super::processor::Processor;

pub trait PermLoad {
    fn session_check_http(&mut self) -> Result<&mut Self, ErrState>;
}

impl PermLoad for Processor {
    fn session_check_http(&mut self) -> Result<&mut Self, ErrState> {
        self.state.stage = 1;
        let mut user_ruid = String::new();
        let mut user_perm = vec![];
        let mut session_id = self.state.session_id.clone();

        if let Some(session_vec) = &self.state.session_id {
            // セッションIDがある場合
            match self.app_set.session.user(session_vec.clone()) {
            Ok(Some(user_ruid_val)) => {
                // セッションIDからユーザーIDを取得
                if let Err(e) = self
                .app_set
                .session
                .update_last_access_time(session_vec.clone())
                {
                return Err(ErrState::new(200, "セッションの最終アクセス時間の更新に失敗".to_string(), Some(e)));
                }
                if let Err(e) = self.app_set.user.update_last_access_time(&user_ruid_val) {
                return Err(ErrState::new(201, "ユーザーの最終アクセス時間の更新に失敗".to_string(), Some(e)));
                }
                match self.app_set.user.get(&user_ruid_val) {
                Ok(user_data) => {
                    user_ruid = format!("{:x}", user_ruid_val);
                    user_perm = user_data.perm;
                }
                Err(e) => {
                    return Err(ErrState::new(202, "ユーザーデータの取得に失敗".to_string(), Some(e)));
                }
                }
            }
            Ok(None) => {
                // セッションIDが無効な場合 (実際は起こりえない)
                match self.app_set.session.set() {
                Ok(new_session_vec) => {
                    let guest_user_ruid = self
                    .app_set
                    .ruid
                    .generate(self.app_set.config.ruid_prefix.USER_EXAMPLE_ID, None);
                    let everyone_permission: u128 =
                    (self.app_set.config.ruid_prefix.USER_EXAMPLE_ID as u128) << 112;
                    if let Err(e) = self.app_set.user.set(
                    &guest_user_ruid.to_u128(),
                    &format!("@guest{}", guest_user_ruid.to_string()),
                    &0,
                    &vec![everyone_permission],
                    ) {
                    return Err(ErrState::new(203, "ゲストユーザーの設定に失敗".to_string(), Some(e)));
                    }
                    // 情報をステートにコピー
                    user_ruid = format!("{:x}", guest_user_ruid.to_u128());
                    session_id = Some(new_session_vec);
                    match self.app_set.user.get(&guest_user_ruid.to_u128()) {
                    Ok(user_data) => {
                        user_perm = user_data.perm;
                    }
                    Err(e) => {
                        return Err(ErrState::new(204, "ゲストユーザーデータの取得に失敗".to_string(), Some(e)));
                    }
                    }
                }
                Err(e) => {
                    return Err(ErrState::new(205, "新しいセッションの設定に失敗".to_string(), Some(e)));
                }
                }
            }
            Err(_) => {
                match self.app_set.session.set() {
                    Ok(new_session_vec) => {
                        let guest_user_ruid = self
                        .app_set
                        .ruid
                        .generate(self.app_set.config.ruid_prefix.USER_EXAMPLE_ID, None);
                        let everyone_permission: u128 =
                        (self.app_set.config.ruid_prefix.USER_EXAMPLE_ID as u128) << 112;
                        if let Err(e) = self.app_set.user.set(
                        &guest_user_ruid.to_u128(),
                        &format!("@guest{}", guest_user_ruid.to_string()),
                        &0,
                        &vec![everyone_permission],
                        ) {
                        return Err(ErrState::new(203, "ゲストユーザーの設定に失敗".to_string(), Some(e)));
                        }
                        // 情報をステートにコピー
                        user_ruid = format!("{:x}", guest_user_ruid.to_u128());
                        session_id = Some(new_session_vec);
                        match self.app_set.user.get(&guest_user_ruid.to_u128()) {
                        Ok(user_data) => {
                            user_perm = user_data.perm;
                        }
                        Err(e) => {
                            return Err(ErrState::new(204, "ゲストユーザーデータの取得に失敗".to_string(), Some(e)));
                        }
                        }
                    }
                    Err(e) => {
                        return Err(ErrState::new(205, "新しいセッションの設定に失敗".to_string(), Some(e)));
                    }
                    }
            }
            }
        } else {
            match self.app_set.session.set() {
            Ok(new_session_vec) => {
                let guest_user_ruid = self
                .app_set
                .ruid
                .generate(self.app_set.config.ruid_prefix.USER_EXAMPLE_ID, None);
                let everyone_permission: u128 = (self.app_set.config.ruid_prefix.USER_EXAMPLE_ID as u128) << 112;
                if let Err(e) = self.app_set.user.set(
                &guest_user_ruid.to_u128(),
                &format!("@guest{}", guest_user_ruid.to_string()),
                &0,
                &vec![everyone_permission],
                ) {
                return Err(ErrState::new(207, "ゲストユーザーの設定に失敗".to_string(), Some(e)));
                }
                // 情報をステートにコピー
                user_ruid = format!("{:x}", guest_user_ruid.to_u128());
                session_id = Some(new_session_vec);
                match self.app_set.user.get(&guest_user_ruid.to_u128()) {
                Ok(user_data) => {
                    user_perm = user_data.perm;
                }
                Err(e) => {
                    return Err(ErrState::new(208, "ゲストユーザーデータの取得に失敗".to_string(), Some(e)));
                }
                }
            }
            Err(e) => {
                return Err(ErrState::new(209, "新しいセッションの設定に失敗".to_string(), Some(e)));
            }
            }
        }

        self.state.user_ruid = user_ruid;
        self.state.user_perm = user_perm;
        self.state.session_id = session_id;
        
        Ok(self)
    }
}
