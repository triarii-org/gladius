use log::debug;

use bernardo::*;

use crate::*;

#[test]
fn completion_test_1() {
    let mut full_setup: FullSetup = FullSetup::new("./test_envs/completion_test_1")
        .with_files(["src/main.rs"])
        // .with_frame_based_wait()
        .build();

    // let file = spath!(full_setup.fsf(), "src", "main.rs").unwrap();

    assert!(full_setup.wait_for(|f| f.is_editor_opened()));

    assert_eq!(
        full_setup
            .get_first_editor()
            .unwrap()
            .get_visible_cursor_line_indices()
            .map(|c| c.visible_idx)
            .collect::<Vec<usize>>(),
        vec![1]
    );

    for _ in 0..4 {
        assert!(full_setup.send_key(Keycode::ArrowDown.to_key()));
    }

    assert!(full_setup.wait_for(|f| f
        .get_first_editor()
        .unwrap()
        .get_visible_cursor_line_indices()
        .map(|c| c.visible_idx)
        .next()
        == Some(5)));

    assert!(full_setup.type_in("path."));
    assert!(full_setup.wait_for(|f| f
        .get_first_editor()
        .unwrap()
        .get_visible_cursor_lines()
        .next()
        .unwrap()
        .contents
        .text
        .contains("path.")));

    full_setup.navcomp_pilot().completions().unwrap().push(MockCompletionMatcher {
        path: None,
        answer: Some(vec![
            Completion {
                key: "into_os_string".to_string(),
                desc: None,
                action: CompletionAction::Insert("into_os_string".to_string()),
            },
            Completion {
                key: "into_boxed_path".to_string(),
                desc: None,
                action: CompletionAction::Insert("into_boxed_path".to_string()),
            },
        ]),
    });

    assert!(full_setup.send_key(Keycode::Space.to_key().with_ctrl()));

    // full_setup.navcomp_pilot().todo_navcomp_sender().

    assert!(full_setup.wait_for(|full_setup| {
        full_setup
            .get_first_editor()
            .unwrap()
            .completions()
            .map(|comp| comp.items().fold(false, |acc, item| acc || item.text.contains("into")))
            .unwrap_or(false)
    }));

    assert!(full_setup.send_key(Keycode::ArrowDown.to_key()));

    // full_setup.screenshot();

    assert!(full_setup.wait_for(|full_setup| {
        full_setup
            .get_first_editor()
            .unwrap()
            .completions()
            .map(|comp| comp.highlighted(true).unwrap().1.trim() == "into_boxed_path")
            .unwrap_or(false)
    }));

    assert!(full_setup.send_key(Keycode::Enter.to_key()));
    assert!(full_setup.wait_for(|f| f
        .get_first_editor()
        .unwrap()
        .get_visible_cursor_lines()
        .next()
        .unwrap()
        .contents
        .text
        .contains("path.into_boxed_path")));
    assert!(full_setup.wait_for(|f| f
        .get_first_editor()
        .unwrap()
        .get_visible_cursor_lines_with_coded_cursors()
        .next()
        .map(|c| {
            debug!("c [{:?}]", c.contents.text);
            c.contents.text == "path.into_boxed_path#⏎"
        })
        .unwrap_or(false)));

    full_setup.finish();
}
