use chrono::{DateTime, Utc};
use common::ToDo;
use sample_todo_yew::todo::{
    self, ActionType, FormState, Msg, TaskError, get_todo, manage_action_request,
};
use sample_todo_yew::todo::{ToDoListProps, ToDoState};
use std::rc::Rc;
use todo::Task;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn handle_action(
    todo: Rc<ToDo>,
    reducer: UseReducerHandle<ToDoState>,
    action_type: &ActionType,
    form_state: UseStateHandle<FormState>,
) {
    let task = match action_type {
        ActionType::Delete => Task::Delete((*todo).clone()),
        ActionType::Add => Task::Add((*todo).clone()),
        ActionType::Update => Task::Update((*todo).clone()),
    };

    let todo_async = todo.clone();
    let reducer_async = reducer.clone();
    let action_type_async = action_type.clone();

    if todo.todo_info.len() == 0 {
        reducer.dispatch(Msg::Error(TaskError::GenericError(
            "ToDo cannot be empty".to_string(),
        )));
    } else {
        reducer.dispatch(Msg::OnGoing(task.clone()));
        wasm_bindgen_futures::spawn_local(async move {
            match manage_action_request(action_type_async, (*todo_async).clone()).await {
                Ok(_) => {
                    reducer_async.dispatch(Msg::Done(task.clone()));
                    form_state.set(FormState::Hidden);
                }
                Err(_) => reducer_async.dispatch(Msg::Error(TaskError::DeleteError)),
            }
        });
    }
}

#[function_component(ToDoList)]
fn todo_list(
    ToDoListProps {
        state, form_state, ..
    }: &ToDoListProps,
) -> Html {
    // let dropdown = use_state(|| false);
    // let toggle = {
    //     let dropdown = dropdown.clone();
    //     Callback::from(move |_| dropdown.set(!*dropdown))
    // };
    // let update_todo = use_state(|| None);
    // let on_todo_update = {
    //     Callback::from(move |todo: ToDo| {
    //         update_todo.set(Some(todo));
    //     })
    // };

    if state.loading {
        return html! {
            <div>
                <h3>{"Loading ..."}</h3>
            </div>
        };
    } else {
        match **form_state {
            FormState::Hidden => {
                html! {
                        <>
                        <div class="flex  justify-center items-center py-6" >
                            <h2 class="mb-4 text-4xl font-bold tracking-tight text-heading md:text-5xl lg:text-3xl">{"ToDos"}</h2>
                        </div>
                        <div class="flex  justify-center items-center py-3">
                        <table class="w-4/5 bg-sky-100 text-sm text-left text-gray-500 dark:text-gray-400">
                            <th scope="col" class="px-4 py-3">{"Date"}</th>
                            <th scope="col" class="px-4 py-3">{"Note"}</th>
                            <th scope="col" class="px-4 py-3">{"Delete"}</th>
                                <tbody>
                                    {for state.todos.iter().map(|todo| {
                                        // This shot has to change
                                        let reducer = state.clone();
                                        let todo_rf_on_click = Rc::new((*todo).clone());
                                        html!{

                                        <tr class="border-b dark:border-gray-700">
                                        <td class="px-4 py-3">{todo.todo_date.clone()}</td>
                                        <td class="px-4 py-3">{todo.todo_info.clone()}</td>

                                        // <td class="px-4 py-3">{get_button(ActionType::Delete, todo_rf_on_click.clone(), reducer.clone())}</td>
                                        // <td class="px-4 py-3">{get_button(ActionType::Update, todo_rf_on_click.clone(), reducer.clone())}</td>
                                        // <td class="px-4 py-3 flex">
                                        //     <button
                                        //       onclick={toggle.clone()}
                                        //       data-popover-target="menu"
                                        //       class="rounded-md  px-4 py-3 border border-transparent text-center text-sm  transition-all shadow-md hover:shadow-lg focus:bg-gray-200 focus:shadow-none active:bg-slate-700 hover:bg-gray-200 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2" type="button">
                                        //         {"..."}
                                        //     </button>

                                        // </td>
                                        <td>
                                            <div class="flex items-center justify-center">
                                                {get_button(ActionType::Delete, todo_rf_on_click.clone(), reducer.clone(),form_state.clone())}
                                            </div>
                                        </td>
                                     </tr>
                                }})}
                                </tbody>
                        </table>
                    </div>
                    </>
                }
            }
            _ => html! {<></>},
        }
    }
}

// This Function for update in the Update Form Component or Add
fn get_button(
    action_type: ActionType,
    todo: Rc<ToDo>,
    reducer: UseReducerHandle<ToDoState>,
    form_state: UseStateHandle<FormState>,
) -> Html {
    match action_type {
        ActionType::Delete => {
            html! {
                <div>
                    <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| {
                        handle_action(todo.clone(), reducer.clone(), &action_type, form_state.clone());
                    }}>
                         <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21
                                   c.342.052.682.107 1.022.166m-1.022-.165L18.16
                                   19.673a2.25 2.25 0 01-2.244 2.077H8.084
                                   a2.25 2.25 0 01-2.244-2.077L4.772
                                   5.79m14.456 0a48.108 48.108 0 00-3.478-.397
                                   m-12 .562c.34-.059.68-.114 1.022-.165m0 0
                                   a48.11 48.11 0 013.478-.397m7.5 0v-.916
                                   c0-1.18-.91-2.164-2.09-2.201a51.964
                                   51.964 0 00-3.32 0c-1.18.037-2.09
                                   1.022-2.09 2.201v.916m7.5 0a48.667
                                   48.667 0 00-7.5 0"
                            />
                        </svg>
                    </button>
                </div>
            }
        }
        ActionType::Update => {
            html! {
                <div>
                    <button class="bg-green-600 hover:bg-green-800 text-white font-bold py-2 px-4 rounded" onclick={move |_| {
                    handle_action(todo.clone(), reducer.clone(), &action_type,form_state.clone());
                }}>{"Update"}
                </button>
                </div>
            }
        }
        ActionType::Add => {
            html! {
                <div>
                <button class="bg-pink-500 hover:bg-pink-700 text-white font-bold py-2 px-4 rounded" onclick={move |_| {
                    handle_action(todo.clone(), reducer.clone(), &action_type, form_state.clone());
                }}>{"Add"}
                </button>
                </div>
            }
        }
    }
}

#[function_component(AddToDoNote)]
fn add_todo(
    ToDoListProps {
        state, form_state, ..
    }: &ToDoListProps,
) -> Html {
    let new_todo = use_state(|| ToDo {
        todo_info: "".to_string(),
        todo_date: "".to_string(),
        id: state.todos.len(),
    });

    let onclick = {
        let form_state = form_state.clone();
        let state = state.clone();
        Callback::from(move |_| match *form_state {
            FormState::Hidden => form_state.set(FormState::Visible(ActionType::Add)),
            _ => {
                state.dispatch(Msg::Done(Task::Loaded(state.todos.to_vec())));
                form_state.set(FormState::Hidden);
            }
        })
    };

    // Handle input changes
    let on_info_change = {
        let new_todo = new_todo.clone();
        Callback::from(move |e: InputEvent| {
            let date: DateTime<Utc> = Utc::now();
            let mut updated = (*new_todo).clone();
            let input: HtmlInputElement = e.target_unchecked_into();
            updated.todo_info = input.value();
            updated.todo_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
            new_todo.set(updated);
        })
    };

    match **form_state {
        FormState::Hidden => {
            return html! {
                <div class="flex py-6 justify-end px-60" >
                    <button class="bg-pink-500 right-0 hover:bg-pink-700 text-white font-bold py-2 px-4 rounded" {onclick}>{"Add ToDo"}</button>
                </div>
            };
        }
        FormState::Visible(ActionType::Add) => {
            return html! {
                <>
                <div class="py-7">
                    <button {onclick}
                        class="fixed top-4 left-4 z-50
                               inline-flex items-center justify-center
                               p-2 rounded-full
                               bg-white shadow
                               text-gray-700 hover:bg-gray-100
                               transition-colors"
                        aria-label="Go back"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-5 h-5"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18"
                            />
                        </svg>
                    </button>
                </div>
                <div class="flex py-6 justify-center" >
                    <h2 class="mb-4 text-4xl font-bold tracking-tight text-heading md:text-5xl lg:text-3xl">{"Add Note"}</h2>
                </div>

                <div class="flex justify-center">
                    <textarea value={new_todo.todo_info.clone()} oninput={on_info_change}
                        rows="10"
                        class="w-2/3 h-60 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand p-3.5 shadow-xs placeholder:text-body resize-y" placeholder="Add your ToDo.."/>
                </div>

                <div class="flex justify-center py-2">
                    {get_button(ActionType::Add, Rc::new((*new_todo).clone()), state.clone(), form_state.clone())}
                </div>
                </>

            };
        }
        _ => {
            html! {
            <div role="alert" class="relative flex w-full items-start rounded-md border border-red-500 bg-red-500 p-2 text-red-50">

                <div class="m-1.5 w-full font-sans text-base leading-none">{"Something Went Wrong"}</div>

              </div>
            }
        }
    }
}

#[function_component(UpdateToDo)]
fn update_todo(todo: &ToDo) -> Html {
    html! {
        <div>
            <label>{"Note"}</label>
            <input type={"text"} minlength={"4"} value={todo.todo_info.clone()}/>
            <div>
                // Missing Button
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let reducer = use_reducer(|| ToDoState {
        todos: Rc::new(vec![]),
        loading: true,
        error: None,
    });

    let form_state = use_state(|| FormState::new());

    let selected_todo = use_state(|| None);
    let on_todo_select = {
        let selected_video = selected_todo.clone();
        Callback::from(move |todo: ToDo| selected_video.set(Some(todo)))
    };

    {
        let reducer = reducer.clone();
        use_effect_with((), move |_| {
            reducer.dispatch(Msg::OnGoing(Task::Loaded(vec![])));
            wasm_bindgen_futures::spawn_local(async move {
                match get_todo().await {
                    Ok(todos_list_props) => {
                        reducer.dispatch(Msg::Done(Task::Loaded(todos_list_props)))
                    }
                    Err(_) => reducer.dispatch(Msg::Error(TaskError::LoadError)),
                }
            });
            || ()
        });
    }

    html! {
        <>
        <div class="bg-blue-500 text-white p-4 rounded space-y-2">
            <h1 class="mb-4 text-4xl text-center font-bold tracking-tight text-heading md:text-2xl lg:text-4xl"> {"ToDo App"} </h1>
        </div>
        {
                    if let Some(error) = reducer.error.clone() {
                        html!{
                            <div class="fixed right-4 max-w-sm w-full bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded shadow-lg">
                                <strong class="font-bold">{"Error: "}</strong>
                                <span class="block sm:inline">{error}</span>
                            </div>
                        }
                    } else {
                        html!{}
                    }
            }
            <div class="">
                <ToDoList state={reducer.clone()} form_state={form_state.clone()} on_click={on_todo_select.clone()}/>
            </div>

            // <div>
            //     if let Some(todo) = &*selected_todo{
            //         // <UpdateToDo todo_info={todo.todo_info} />
            //     }
            // </div>
            <div>

               <AddToDoNote state={reducer.clone()} form_state={form_state.clone()} on_click={on_todo_select.clone()}/>
            </div>

        </>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

// {
//                                     if *dropdown {
//                                         html!{
//                                             <ul
//                                               role="menu"
//                                               data-popover="menu"
//                                               data-popover-placement="bottom"
//                                               class="absolute z-10 min-w-[100px] bg-sky-50 overflow-auto rounded-lg border border-slate-200  p-1.5 shadow-lg shadow-sm focus:outline-none"
//                                             >
//                                               <li
//                                                 role="menuitem"
//                                                 class="cursor-pointer text-slate-800 flex w-full text-sm items-center rounded-md p-3 transition-all hover:bg-slate-100 focus:bg-slate-100 active:bg-slate-100"
//                                               >
//                                                 {"Update"}
//                                               </li>
//                                               <li
//                                                 role="menuitem"
//                                                 class="cursor-pointer text-slate-800 flex w-full text-sm items-center rounded-md p-3 transition-all hover:bg-slate-100 focus:bg-slate-100 active:bg-slate-100"
//                                                 onclick={move |_| {
//                                                     handle_action(todo_rf_on_click.clone(), reducer.clone(), &ActionType::Delete,form_state_inner.clone());
//                                                 }}
//                                               >
//                                                 <p class="text-red-600">
//                                                 {"Delete"}
//                                                 </p>
//                                               </li>
//
//                                             </ul>
//                                         }
//                                     } else {
//                                         html! {}
//                                     }
//                                 }
//                                 </tr>
