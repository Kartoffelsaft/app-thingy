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
        .then(() => fetch('/api/', {method: 'GET'}))
        .then(currentItems => {
            return currentItems.json();
        })
        .then(currentItems => {
            itemList.innerHTML = "";
            currentItems.value.forEach((item: any) => {
                var entry = document.createElement('p');
                entry.className = 'item-entry';
                entry.innerText = item.text;
                itemList.appendChild(entry);
                
                let date_created = new Date(0);
                date_created.setUTCSeconds(item.time_created);
                console.log(date_created);
            });
        })
        .then(() => inputBox.value = "");
}
