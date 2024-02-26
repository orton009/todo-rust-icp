import React, { useState, useEffect } from 'react';
import { my_project_backend as backend } from 'declarations/my_project_backend';

function TodoApp() {
  const [todos, setTodos] = useState([]);
  const [newTodoText, setNewTodoText] = useState('');
  const [pageNumber, setPageNumber] = useState(1);
  const [pageSize, setPageSize] = useState(5);

  useEffect(() => {
    async function fetchData() {
      const result = await backend.get_all_todos(pageNumber, pageSize);
      setTodos(JSON.parse(result));
      console.log("fetch result", result)
    }
    fetchData();
  }, [pageNumber, pageSize]);

  const handleCreateTodo = async () => {
    if (newTodoText.trim() !== '') {
      const result = await backend.create_todo(JSON.stringify({text: newTodoText}));
      console.log("create result", result)
      if (result) {
        setNewTodoText('');
        setPageNumber(1); // Refresh todo list after creating a new todo
      }
    }
  };

  const handleUpdateTodo = async (todo) => {
    const result = await backend.update_todo(JSON.stringify(todo));
    console.log("update result", result)
    if (result) {
      setPageNumber(1); // Refresh todo list after updating a todo
    }
  };

  const handleDeleteTodo = async (id) => {
    const result = await backend.delete_todo(id);
    console.log("delete result", result)
    if (result) {
      setPageNumber(1); // Refresh todo list after deleting a todo
    }
  };

  return (
    <div>
      <h1>Todos</h1>
      <input type="text" value={newTodoText} onChange={(e) => setNewTodoText(e.target.value)} />
      <button onClick={handleCreateTodo}>Add Todo</button>
      <ul>
        {todos.map(todo => (
          <li key={todo.id}>
            <input value={todo.text} onChange={ev => setTodos(todos.map(t => t.id == todo.id ? {text: ev.currentTarget["value"], id: todo.id} : t))}/>
            <button onClick={() => handleUpdateTodo(todo)}>Update</button>
            <button onClick={() => handleDeleteTodo(todo.id)}>Delete</button>
          </li>
        ))}
      </ul>
      <button onClick={() => setPageNumber(pageNumber - 1)} disabled={pageNumber === 1}>Previous Page</button>
      <button onClick={() => setPageNumber(pageNumber + 1)}>Next Page</button>
    </div>
  );
}

export default TodoApp;
