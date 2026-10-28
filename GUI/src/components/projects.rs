use dioxus::html::FileData;
use dioxus::prelude::*;
use tetrislib::dto::model::Layout;
use tetrislib::dto::project_request::ProjectRequest;
use tetrislib::dto::solution_request::SolutionRequest;
use tetrislib::store;

#[derive(Clone, PartialEq)]
pub struct LayoutSelection {
    pub solution: String,
    pub name: String,
    pub output: bool,
}

#[derive(Clone, PartialEq)]
enum Editor {
    AddProject,
    EditProject {
        id: String,
        name: String,
        description: String,
    },
    AddSolution {
        project: String,
        project_name: String,
    },
    AddInput {
        solution: String,
        solution_name: String,
    },
    EditSolution {
        id: String,
        project: String,
        name: String,
        version: String,
    },
}

#[derive(Clone, PartialEq)]
enum DeleteTarget {
    Project {
        id: String,
        name: String,
        solutions: Vec<String>,
    },
    Solution {
        id: String,
        name: String,
    },
}

#[component]
pub fn ProjectsPanel(
    selected: Option<LayoutSelection>,
    onselect: EventHandler<LayoutSelection>,
    onclear: EventHandler,
) -> Element {
    let mut projects = use_resource(store::all_projects);
    let mut editor = use_signal(|| None::<Editor>);
    let mut deleting = use_signal(|| None::<DeleteTarget>);
    let state = projects.cloned();

    rsx! {
        aside { class: "projects-panel", aria_label: "Projects and solutions",
            div { class: "projects-heading",
                h2 { "Projects" }
                button {
                    class: "tool-button", title: "Refresh projects", aria_label: "Refresh projects",
                    onclick: move |_| projects.restart(), "↻"
                }
            }
            button { class: "primary-button add-project",
                onclick: move |_| editor.set(Some(Editor::AddProject)), "+ Add project"
            }
            p { class: "muted layout-hint", "Each solution has one input layout and one processed output." }
            match state {
                None => rsx! { p { class: "muted", role: "status", "Loading projects…" } },
                Some(Err(message)) => rsx! {
                    div { class: "panel-error", role: "alert",
                        p { "Could not load projects." }
                        p { "{message}" }
                        button { class: "tool-button", onclick: move |_| projects.restart(), "Retry" }
                    }
                },
                Some(Ok(items)) => rsx! {
                    if items.is_empty() {
                        p { class: "muted", "No projects yet. Add a project to get started." }
                    }
                    ul { class: "project-tree",
                        for project in items {
                            li { key: "{project.id}", class: "project-node",
                                details { open: true,
                                    summary {
                                        span { class: "project-name", "{project.name}" }
                                        span { class: "solution-count", "{project.solutions.len()}" }
                                    }
                                    if !project.description.is_empty() {
                                        p { class: "project-description", "{project.description}" }
                                    }
                                    div { class: "project-actions",
                                        button { class: "text-button",
                                            onclick: {
                                                let project = project.clone();
                                                move |_| editor.set(Some(Editor::AddSolution {
                                                    project: project.id.to_string(),
                                                    project_name: project.name.clone(),
                                                }))
                                            }, "+ Add solution"
                                        }
                                        button { class: "text-button",
                                            onclick: {
                                                let project = project.clone();
                                                move |_| editor.set(Some(Editor::EditProject {
                                                    id: project.id.to_string(),
                                                    name: project.name.clone(),
                                                    description: project.description.clone(),
                                                }))
                                            }, "Edit"
                                        }
                                        button { class: "text-button danger-text",
                                            onclick: {
                                                let target = DeleteTarget::Project { id: project.id.to_string(), name: project.name.clone(), solutions: project.solutions.iter().map(|s| s.id.to_string()).collect() };
                                                move |_| deleting.set(Some(target.clone()))
                                            }, "Delete"
                                        }
                                    }
                                    if project.solutions.is_empty() {
                                        p { class: "muted empty-solutions", "No solutions yet" }
                                    }
                                    ul { class: "solution-tree",
                                        for solution in project.solutions.iter() {
                                            li { key: "{solution.id}", class: "solution-node",
                                                div { class: "solution-heading",
                                                    span { "{solution.name}" }
                                                    span { class: "solution-version", "{solution.version}" }
                                                }
                                                div { class: "solution-actions",
                                                    button { class: "text-button",
                                                        onclick: {
                                                            let action = Editor::EditSolution { id: solution.id.to_string(), project: project.id.to_string(), name: solution.name.clone(), version: solution.version.clone() };
                                                            move |_| editor.set(Some(action.clone()))
                                                        }, "Edit"
                                                    }
                                                    button { class: "text-button danger-text",
                                                        onclick: {
                                                            let target = DeleteTarget::Solution { id: solution.id.to_string(), name: solution.name.clone() };
                                                            move |_| deleting.set(Some(target.clone()))
                                                        }, "Delete"
                                                    }
                                                }
                                                ul { class: "layout-tree",
                                                    li {
                                                        button { class: "layout-entry",
                                                            disabled: solution.input_path.is_empty(),
                                                            aria_pressed: selected.as_ref().is_some_and(|s| s.solution == solution.id.to_string() && !s.output),
                                                            onclick: {
                                                                let selection = LayoutSelection { solution: solution.id.to_string(), name: solution.name.clone(), output: false };
                                                                move |_| onselect.call(selection.clone())
                                                            },
                                                            span { "↳ Input" }
                                                            if solution.input_path.is_empty() { span { class: "muted", "Not uploaded" } }
                                                        }
                                                        if solution.input_path.is_empty() {
                                                            button { class: "text-button",
                                                                onclick: {
                                                                    let action = Editor::AddInput { solution: solution.id.to_string(), solution_name: solution.name.clone() };
                                                                    move |_| editor.set(Some(action.clone()))
                                                                }, "+ Add input"
                                                            }
                                                        }
                                                    }
                                                    if solution.output_path.is_some() {
                                                        li {
                                                            button { class: "layout-entry",
                                                                aria_pressed: selected.as_ref().is_some_and(|s| s.solution == solution.id.to_string() && s.output),
                                                                onclick: {
                                                                    let selection = LayoutSelection { solution: solution.id.to_string(), name: solution.name.clone(), output: true };
                                                                    move |_| onselect.call(selection.clone())
                                                                }, "↳ Output"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
        if let Some(action) = editor() {
            ProjectEditor {
                action,
                selected: selected.clone(),
                onclose: move |_| { editor.set(None); projects.restart(); },
                onsaved: move |selection| {
                    editor.set(None);
                    projects.restart();
                    if let Some(selection) = selection { onselect.call(selection); }
                },
            }
        }
        if let Some(target) = deleting() {
            DeleteDialog {
                target,
                onclose: move |_| deleting.set(None),
                ondeleted: {
                    let selected = selected.clone();
                    move |ids: Vec<String>| {
                        if selected.as_ref().is_some_and(|s| ids.contains(&s.solution)) { onclear.call(()); }
                        deleting.set(None);
                        projects.restart();
                    }
                },
            }
        }
    }
}

#[component]
fn ProjectEditor(
    action: Editor,
    selected: Option<LayoutSelection>,
    onclose: EventHandler,
    onsaved: EventHandler<Option<LayoutSelection>>,
) -> Element {
    let initial_name = match &action {
        Editor::EditProject { name, .. } | Editor::EditSolution { name, .. } => name.clone(),
        _ => String::new(),
    };
    let initial_description = match &action {
        Editor::EditProject { description, .. } => description.clone(),
        _ => String::new(),
    };
    let mut name = use_signal(|| initial_name);
    let mut description = use_signal(|| initial_description);
    let initial_version = match &action {
        Editor::EditSolution { version, .. } => version.clone(),
        _ => "1.0".to_string(),
    };
    let mut version = use_signal(|| initial_version);
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut file = use_signal(|| None::<FileData>);
    // Retain the created ID if uploading fails, so retry never creates a duplicate solution.
    let mut created_solution = use_signal(|| None::<String>);
    let is_solution = matches!(
        &action,
        Editor::AddSolution { .. } | Editor::EditSolution { .. }
    );
    let input_only = matches!(&action, Editor::AddInput { .. });
    let needs_file = matches!(&action, Editor::AddSolution { .. }) || input_only;
    let title = match &action {
        Editor::AddProject => "Add project",
        Editor::EditProject { .. } => "Edit project",
        Editor::AddSolution { .. } => "Add solution",
        Editor::AddInput { .. } => "Add input layout",
        Editor::EditSolution { .. } => "Edit solution",
    };
    let project_name = match &action {
        Editor::AddSolution { project_name, .. } => Some(project_name.clone()),
        _ => None,
    };

    rsx! {
        dialog { id: "project-editor", class: "editor-dialog", aria_labelledby: "editor-title",
            onmounted: move |_| { document::eval("document.getElementById('project-editor').showModal()"); },
            oncancel: move |event| {
                event.prevent_default();
                if !saving() { onclose.call(()); }
            },
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                    if saving() { return; }
                    let name = name().trim().to_string();
                    let description = description();
                    let version = version().trim().to_string();
                    if (!input_only && name.is_empty()) || (is_solution && version.is_empty()) {
                        error.set(Some("Please complete the required fields.".to_string()));
                        return;
                    }
                    saving.set(true);
                    error.set(None);
                    let action = action.clone();
                    let selected = selected.clone();
                    let selected_file = file();
                    spawn(async move {
                        let bytes = if needs_file {
                            let Some(file) = selected_file else {
                                saving.set(false);
                                error.set(Some("Choose an input layout JSON file.".to_string()));
                                return;
                            };
                            if file.size() > 16 * 1024 * 1024 {
                                saving.set(false);
                                error.set(Some("The input layout must be no larger than 16 MiB.".to_string()));
                                return;
                            }
                            match file.read_bytes().await {
                                Ok(bytes) => {
                                    if let Err(problem) = serde_json::from_slice::<Layout>(&bytes) {
                                        saving.set(false);
                                        error.set(Some(format!("This file is not a Layout: {problem}")));
                                        return;
                                    }
                                    bytes.to_vec()
                                },
                                Err(problem) => {
                                    saving.set(false);
                                    error.set(Some(format!("Cannot read the selected file: {problem}")));
                                    return;
                                },
                            }
                        } else { Vec::new() };
                        let result = match action {
                            Editor::AddProject => store::create_project(ProjectRequest {
                                id: None, name, description,
                            }).await.map(|_| None),
                            Editor::EditProject { id, .. } => match id.parse() {
                                Ok(id) => store::update_project(ProjectRequest {
                                    id: Some(id), name, description,
                                }).await.map(|_| None),
                                Err(_) => Err("Invalid project ID.".to_string()),
                            },
                            Editor::AddSolution { project, .. } => {
                                let id = if let Some(id) = created_solution() { Ok(id) } else {
                                    match project.parse() {
                                        Ok(project) => store::create_solution(SolutionRequest {
                                            id: None, project, name: name.clone(), version,
                                            input_path: String::new(), output_path: None,
                                        }).await.map(|solution| {
                                            let id = solution.id.to_string();
                                            created_solution.set(Some(id.clone()));
                                            id
                                        }),
                                        Err(_) => Err("Invalid project ID.".to_string()),
                                    }
                                };
                                match id {
                                    Ok(id) => store::upload_solution_source(&id, bytes).await
                                        .map(|_| Some(LayoutSelection { solution: id, name, output: false })),
                                    Err(error) => Err(error),
                                }
                            },
                            Editor::AddInput { solution, solution_name } => store::upload_solution_source(&solution, bytes).await
                                .map(|_| Some(LayoutSelection { solution, name: solution_name, output: false })),
                            Editor::EditSolution { id, project, .. } => match (id.parse(), project.parse()) {
                                (Ok(uuid), Ok(project)) => store::update_solution(SolutionRequest {
                                    id: Some(uuid), project, name: name.clone(), version,
                                    input_path: String::new(), output_path: None,
                                }).await.map(|_| selected.filter(|s| s.solution == id).map(|s| LayoutSelection { name, ..s })),
                                _ => Err("Invalid solution or project ID.".to_string()),
                            },
                        };
                        saving.set(false);
                        match result {
                            Ok(selection) => onsaved.call(selection),
                            Err(message) => error.set(Some(message)),
                        }
                    });
                },
                div { class: "editor-heading",
                    h2 { id: "editor-title", "{title}" }
                    button { r#type: "button", class: "text-button", aria_label: "Close dialog",
                        disabled: saving(), onclick: move |_| onclose.call(()), "✕"
                    }
                }
                if let Some(project_name) = project_name {
                    p { class: "muted", "Project: {project_name}" }
                }
                fieldset { disabled: saving(),
                    if !input_only {
                    label { r#for: "editor-name", "Name *" }
                    input { id: "editor-name", name: "name", required: true, autofocus: true,
                        disabled: created_solution().is_some(),
                        value: "{name}", oninput: move |event| name.set(event.value())
                    }
                    }
                    if is_solution {
                        label { r#for: "editor-version", "Version *" }
                        input { id: "editor-version", name: "version", required: true,
                            disabled: created_solution().is_some(),
                            value: "{version}", oninput: move |event| version.set(event.value())
                        }
                    } else if !input_only {
                        label { r#for: "editor-description", "Description" }
                        textarea { id: "editor-description", name: "description", rows: "4",
                            value: "{description}", oninput: move |event| description.set(event.value())
                        }
                    }
                    if needs_file {
                        p { class: "muted", "One solution pairs one input layout with one processed output. The output appears when processing finishes." }
                        label { r#for: "editor-source", "Input layout (JSON) *" }
                        input { id: "editor-source", r#type: "file", accept: ".json,application/json", required: true,
                            autofocus: input_only,
                            onchange: move |event| { file.set(event.files().into_iter().next()); error.set(None); }
                        }
                        p { class: "muted", "Maximum file size: 16 MiB." }
                    }
                }
                if created_solution().is_some() {
                    p { class: "muted", role: "status", "Solution created. If the upload fails, retry here or add its input later from the tree." }
                }
                if let Some(message) = error() {
                    p { class: "form-error", role: "alert", "{message}" }
                }
                div { class: "editor-actions",
                    button { r#type: "button", class: "tool-button", disabled: saving(),
                        onclick: move |_| onclose.call(()), "Cancel"
                    }
                    button { r#type: "submit", class: "primary-button", disabled: saving(),
                        if saving() { "Saving…" } else { "Save" }
                    }
                }
            }
        }
    }
}

#[component]
fn DeleteDialog(
    target: DeleteTarget,
    onclose: EventHandler,
    ondeleted: EventHandler<Vec<String>>,
) -> Element {
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let (kind, name) = match &target {
        DeleteTarget::Project { name, .. } => ("project", name.clone()),
        DeleteTarget::Solution { name, .. } => ("solution", name.clone()),
    };
    let is_project = matches!(&target, DeleteTarget::Project { .. });
    rsx! {
        dialog { id: "delete-dialog", class: "editor-dialog", aria_labelledby: "delete-title",
            onmounted: move |_| { document::eval("document.getElementById('delete-dialog').showModal()"); },
            oncancel: move |event| { event.prevent_default(); if !busy() { onclose.call(()); } },
            h2 { id: "delete-title", "Delete {kind}?" }
            p { "Delete “{name}”?" }
            if is_project { p { class: "muted", "This also removes all solutions in this project." } }
            if let Some(message) = error() { p { class: "form-error", role: "alert", "{message}" } }
            div { class: "editor-actions",
                button { class: "tool-button", autofocus: true, disabled: busy(), onclick: move |_| onclose.call(()), "Cancel" }
                button { class: "danger-button", disabled: busy(),
                    onclick: move |_| {
                        if busy() { return; }
                        busy.set(true);
                        error.set(None);
                        let target = target.clone();
                        spawn(async move {
                            let result = match target {
                                DeleteTarget::Project { id, solutions, .. } => store::delete_project(&id).await.map(|_| solutions),
                                DeleteTarget::Solution { id, .. } => store::delete_solution(&id).await.map(|_| vec![id]),
                            };
                            busy.set(false);
                            match result { Ok(ids) => ondeleted.call(ids), Err(message) => error.set(Some(message)) }
                        });
                    },
                    if busy() { "Deleting…" } else { "Delete" }
                }
            }
        }
    }
}
