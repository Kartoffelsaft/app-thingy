import prettyMilliseconds from 'pretty-ms';

var inputMemo = document.getElementById("memo") as HTMLInputElement;
var inputEventDate = document.getElementById("event") as HTMLInputElement;
var itemList = document.getElementById("items") as HTMLDivElement;
var composeButton = document.getElementById("compose-button") as HTMLButtonElement;
var composeOverlay = document.getElementById("compose-overlay") as HTMLDivElement;
var composeForm = document.getElementById("compose-form") as HTMLFormElement;

class RawItem {
    memo: string;
    created_time: string;
    event_time: string;

    constructor(nmemo: string, ncreated_time: string, nevent_time: string) {
        this.memo = nmemo;
        this.created_time = ncreated_time;
        this.event_time = nevent_time;
    }
}

class Item {
    memo: string;
    created_time: Date;
    event_time: Date;

    constructor(raw: RawItem) {
        this.memo = raw.memo;
        this.created_time = new Date(raw.created_time);
        this.event_time = new Date(raw.event_time);
    }
}

var data: Array<Item>;

const createEntryFromItem = (item: Item) => {
    let time = item.event_time.getTime() - Date.now();
    let humanReadableTime: string;
    if(time < 0) {
        humanReadableTime = prettyMilliseconds(-time, {compact: true}) + " ago";
    } else {
        humanReadableTime = "in " + prettyMilliseconds(time, {compact: true});
    }

    let entry: HTMLElement = document.createElement('p');
    entry.className = 'item-entry';
    entry.innerText = item.memo + " " + humanReadableTime;

    return entry;
}

composeButton.onclick = () => {
    composeOverlay.style.display = "grid";
}

composeForm.onsubmit = () => {
    let date = new Date(inputEventDate.value);
    fetch('/api/', {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({"memo":inputMemo.value, "event_time":date.toJSON()})
    })
        .then(() => fetch('/api/', {method: 'GET'}))
        .then(currentItems => currentItems.json())
        .then(currentItems => {
            data = currentItems.value.map((raw: RawItem) => new Item(raw));
            itemList.innerHTML = "";
            data.forEach(item => {
                itemList.appendChild(createEntryFromItem(item));
            });
            console.log(data);
        })
        .then(() => {
            inputMemo.value = "";
            inputEventDate.value = "";
            composeOverlay.style.display = "none";
        });
}
