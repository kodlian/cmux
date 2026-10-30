//! Exercise the real daemon's supervisor and detached-updater shutdown paths.
#![cfg(unix)]
#[path = "support/daemon_env.rs"]
mod daemon_env;
const HERMETIC: daemon_env::Process = daemon_env::Process::deny();

use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_tungstenite::tungstenite::{Message, client::IntoClientRequest};

async fn daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body() {
    let _ = HERMETIC;
    // SIGTERM is the supervisor update path; --restart is the detached update path.
    for trigger in ["sigterm", "restart", "stop"] {
        let dir = cmux_unix_socket::short_test_dir(&format!("amx-close-{trigger}"));
        let socket = dir.path().join("s.sock");
        let mut child = daemon_env::tokio_command(dir.path(), daemon_env::Agents::Deny)
            .env("ACPMUX_HOME", dir.path())
            .env("ACPMUX_SOCKET", &socket)
            .args([
                "daemon",
                "run",
                "--memory",
                "--listen",
                "127.0.0.1:0",
                "--token",
                "test-token",
                "--ready-fd",
                "1",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let stdout = child.stdout.take().unwrap();
        let ready = tokio::time::timeout(Duration::from_secs(30), async {
            let mut lines = BufReader::new(stdout).lines();
            while let Some(line) = lines.next_line().await.unwrap() {
                if let Ok(value) = serde_json::from_str::<Value>(&line)
                    && value["ready"] == true
                {
                    return value;
                }
            }
            panic!("daemon ended before ready");
        })
        .await
        .unwrap();
        let mut request =
            format!("ws://{}", ready["listen"].as_str().unwrap()).into_client_request().unwrap();
        request.headers_mut().insert("authorization", "Bearer test-token".parse().unwrap());
        let (mut ws, _) = tokio_tungstenite::connect_async(request).await.unwrap();
        ws.send(Message::Text(
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1}})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
        assert!(matches!(
            tokio::time::timeout(Duration::from_secs(5), ws.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap(),
            Message::Text(_)
        ));
        let command = if trigger == "sigterm" {
            // Only the child this test created is signaled.
            assert_eq!(unsafe { libc::kill(child.id().unwrap() as i32, libc::SIGTERM) }, 0);
            None
        } else {
            let mut command = daemon_env::tokio_command(dir.path(), daemon_env::Agents::Deny);
            command.env("ACPMUX_HOME", dir.path()).env("ACPMUX_SOCKET", &socket).args([
                "daemon",
                "shutdown",
                "--keep-agents",
            ]);
            if trigger == "restart" {
                command.arg("--restart");
            }
            Some(command.kill_on_drop(true).spawn().unwrap())
        };
        let frame = tokio::time::timeout(Duration::from_secs(10), ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let Message::Close(close) = frame else {
            panic!("{trigger}: expected Close, got {frame:?}")
        };
        let code = close.map(|frame| u16::from(frame.code));
        if trigger == "stop" {
            assert_ne!(code, Some(1012));
        } else {
            assert_eq!(code, Some(1012), "{trigger}");
        }
        assert!(
            tokio::time::timeout(Duration::from_secs(10), child.wait())
                .await
                .unwrap()
                .unwrap()
                .success()
        );
        if let Some(mut command) = command {
            assert!(
                tokio::time::timeout(Duration::from_secs(10), command.wait())
                    .await
                    .unwrap()
                    .unwrap()
                    .success()
            );
        }
    }
}

#[tokio::test]
async fn daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_1() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_2() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_3() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_4() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_5() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_6() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_7() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_8() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_9() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_10() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_11() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_12() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_13() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_14() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_15() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_16() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_17() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_18() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_19() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_20() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_21() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_22() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_23() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_24() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_25() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_26() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_27() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_28() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_29() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_30() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_31() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_32() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_33() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_34() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_35() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_36() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_37() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_38() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_39() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_40() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_41() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_42() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_43() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_44() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_45() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_46() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_47() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_48() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_49() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_50() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_51() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_52() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_53() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_54() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_55() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_56() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_57() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_58() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_59() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_60() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_61() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_62() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_63() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_64() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_65() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_66() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_67() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_68() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_69() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_70() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_71() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_72() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_73() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_74() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_75() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_76() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_77() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_78() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_79() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_80() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_81() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_82() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_83() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_84() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_85() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_86() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_87() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_88() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_89() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_90() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_91() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_92() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_93() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_94() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_95() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_96() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_97() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_98() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_99() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_100() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_101() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_102() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_103() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_104() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_105() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_106() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_107() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_108() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_109() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_110() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_111() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_112() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_113() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_114() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_115() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_116() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_117() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_118() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_119() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}

#[tokio::test]
async fn arr_copy_daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_120() {
    daemon_update_and_sigterm_send_1012_but_explicit_stop_is_normal_body().await;
}
