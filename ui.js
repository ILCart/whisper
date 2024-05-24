window.onload = function () {

  var form = document.getElementById('message-form');
  var messageField = document.getElementById('message');
  var closeBtn = document.getElementById('close');
  let uuid;

  var socket = new WebSocket('ws://127.0.0.1:3001/ws');

  socket.onerror = function (error) {
    console.log('WebSocket Error: ' + error);
  };

  socket.onopen = function (event) {
    console.log('open');
  };

  socket.onmessage = function (event) {
    console.log(event);
    let data = JSON.parse(event.data);
    if (uuid == null) {
      uuid = data.uuid;
    }
    if (data.uuid !== uuid) {
      new_message("outside", data.data.payload)
    }
  };

  socket.onclose = function (event) {
    console.log('open');
  };

  form.onsubmit = function (e) {
    e.preventDefault();

    var message = messageField.value;

    new_message("self", message);

    let data = { "uuid": uuid, "data": { "event": "SEND_MESSAGE", "payload": message } };

    console.log("sent:\n", data);

    socket.send(JSON.stringify(data));

    messageField.value = '';

    return false;
  };

  closeBtn.onclick = function (e) {

    e.preventDefault();
    socket.close();

    return false;
  };

};


function new_message(type, msg) {
  //types: self, outside, self_reply, outside reply
  let messageField = document.querySelector(".ui-msg-history");
  let bubble = document.createElement("div");
  switch (type) {
    case "self":
      console.log(type, msg);
      bubble.classList.add("ui-bubble");
      bubble.classList.add("self");
      bubble.textContent = msg;
      messageField.appendChild(bubble);
      break;
    case "outside":
      console.log(type, msg);
      bubble.classList.add("ui-bubble");
      bubble.classList.add("outside");
      bubble.textContent = msg;
      messageField.appendChild(bubble);
      break;
    case "self_reply":
      console.log(type, msg);
      break;
    case "outside_reply":
      console.log(type, msg);
      break;
  }
}
function testmsg() {
  for (let i = 0; i < 159; i++) {
    new_message("self", `hello${i}`);
    new_message("outside", "hi!");
  }
}

function sort() {
  let numbers = Array.from({ length: 100 }, (_, index) => index + 1);
  for (let i = numbers.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [numbers[i], numbers[j]] = [numbers[j], numbers[i]];
  }
  for (let i = 0; i < numbers.length; i++) {
    numbers.unshift(i);
  }
  console.log(numbers);
}
