var inputBox = document.getElementById("add-item-textbox") as HTMLInputElement;
var itemList = document.getElementById("items") as HTMLDivElement;

class Item {
    text: string;
    time_created: number;
}

var data: Array<Item>;

const createEntryFromItem = (item: Item) => {
    let entry: HTMLElement = document.createElement('p');
    entry.className = 'item-entry';
    entry.innerText = item.text;

    return entry;
}

const newItem = () => {
    fetch('/api/', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({"text":inputBox.value})
    })
        .then(() => fetch('/api/', {method: 'GET'}))
        .then(currentItems => currentItems.json())
        .then(currentItems => {
            data = currentItems.value;
            itemList.innerHTML = "";
            data.forEach(item => {
                itemList.appendChild(createEntryFromItem(item));
                
                let date_created = new Date(0);
                date_created.setUTCSeconds(item.time_created);
                console.log(date_created);
            });
        })
        .then(() => inputBox.value = "");
}
