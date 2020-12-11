var inputBox = document.getElementById("add-item-textbox") as HTMLInputElement;
var itemList = document.getElementById("items") as HTMLDivElement;

const newItem = () => {
    fetch('/api/', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({"text":inputBox.value})
    })
        .then(currentItems => {
            return currentItems.json();
        })
        .then(currentItems => {
            itemList.innerHTML = "";
            currentItems.value.forEach((item: string) => {
                var entry = document.createElement('p');
                entry.className = 'item-entry';
                entry.innerText = item;
                itemList.appendChild(entry);
            });
        })
        .then(() => inputBox.value = "");
}
