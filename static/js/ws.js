const socket = new WebSocket("ws://127.0.0.1:8000/ws/")

socket.onopen = (event) => {
    console.log("WebSocket is open now.");
};

socket.onclose = (event) => {
    console.log("WebSocket is closed now.");
};

socket.onerror = (event) => {
    console.error("WebSocket error observed:", event);
};

socket.onmessage = (event) => {
  alert(event.data)
};

socket.send("lol")
