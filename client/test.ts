var inputBox = document.getElementById("add-item-textbox") as HTMLInputElement;

const newItem = () => {
    fetch('/', {
        method: 'POST', 
        mode: 'cors',
        headers: {
            'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({"text":inputBox.value})
    })
        .then(currentItems => {
            console.log(currentItems);
            return currentItems.json();
        })
        .then(currentItems => console.log(currentItems.value))
        .then(() => inputBox.value = "");
}
