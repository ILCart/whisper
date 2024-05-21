window.onload = function() {

    // Get references to elements on the page.
    var form = document.getElementById('message-form');
    var messageField = document.getElementById('message');
    var messagesList = document.getElementById('messages');
    var socketStatus = document.getElementById('status');
    var closeBtn = document.getElementById('close');
  
  
    // Create a new WebSocket.
    var socket = new WebSocket('ws://127.0.0.1:3001/ws');
  
  
    // Handle any errors that occur.
    socket.onerror = function(error) {
      console.log('WebSocket Error: ' + error);
    };
  
  
    // Show a connected message when the WebSocket is opened.
    socket.onopen = function(event) {
        console.log('open');
    };
  
  
    // Handle messages sent by the server.
    socket.onmessage = function(event) {
        console.log(event.data);
    };
  
  
    // Show a disconnected message when the WebSocket is closed.
    socket.onclose = function(event) {
        console.log('open');
    };
  
  
    // Send a message when the form is submitted.
    form.onsubmit = function(e) {
      e.preventDefault();
  
      // Retrieve the message from the textarea.
      var message = messageField.value;
  
      // Send the message through the WebSocket.
      socket.send(message);
  
      // Clear out the message field.
      messageField.value = '';
  
      return false;
    };
  
  
    // Close the WebSocket connection when the close button is clicked.
    closeBtn.onclick = function(e) {
      e.preventDefault();
  
      // Close the WebSocket.
      socket.close();
  
      return false;
    };
  
  };
  