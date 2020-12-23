import prettyMilliseconds from 'pretty-ms';
import { Calendar } from '@fullcalendar/core';
import dayGridPlugin from '@fullcalendar/daygrid';

var inputMemo = document.getElementById("memo") as HTMLInputElement;
var inputEventDate = document.getElementById("event") as HTMLInputElement;

var composeButton = document.getElementById("compose-button") as HTMLButtonElement;
var composeOverlay = document.getElementById("compose-overlay") as HTMLDivElement;
var composeForm = document.getElementById("compose-form") as HTMLFormElement;

var itemList = document.getElementById("items") as HTMLDivElement;
var calendarElement = document.getElementById("calendar") as HTMLDivElement;
var calendar: Calendar;

document.addEventListener('DOMContentLoaded', () => {
    calendar = new Calendar(calendarElement, {
        plugins: [ dayGridPlugin ]
    });

    calendar.render();
});

class RawItem {
    id: string;
    memo: string;
    created_time: string;
    event_time: string;

    constructor(nid: string, nmemo: string, ncreated_time: string, nevent_time: string) {
        this.id = nid;
        this.memo = nmemo;
        this.created_time = ncreated_time;
        this.event_time = nevent_time;
    }
}

class Item {
    id: string;
    memo: string;
    created_time: Date;
    event_time: Date;

    constructor(raw: RawItem) {
        this.id = raw.id;
        this.memo = raw.memo;
        this.created_time = new Date(raw.created_time);
        this.event_time = new Date(raw.event_time);
    }
}

var data: Array<Item>;

const createEntryFromItem = (item: Item) => {
    let time = item.event_time.getTime() - Date.now() + item.event_time.getTimezoneOffset() * 60 * 1000;
    let humanReadableTime: string;
    if(time < 0) {
        humanReadableTime = prettyMilliseconds(-time, {compact: true}) + " ago";
    } else {
        humanReadableTime = "in " + prettyMilliseconds(time, {compact: true});
    }

    let entryBox: HTMLElement = document.createElement('p');
    entryBox.className = 'item-entry-box';

    let entryTime: HTMLDivElement = document.createElement('div');
    entryTime.className = 'item-entry-time';
    entryTime.innerText = humanReadableTime;

    let entryMemo: HTMLDivElement = document.createElement('div');
    entryMemo.className = 'item-entry-memo';
    entryMemo.innerText = item.memo;

    let entryDeleteButton: HTMLButtonElement = document.createElement('button');
    entryDeleteButton.className = 'item-entry-delete-button';
    entryDeleteButton.innerText = '✖';
    entryDeleteButton.onclick = () => {
        fetch(`/api/${item.id}`, {method: 'DELETE'})
            .then(refreshItems);
    };

    entryBox.appendChild(entryTime);
    entryBox.appendChild(entryMemo);
    entryBox.appendChild(entryDeleteButton);

    return entryBox;
}

const refreshItems = () => {
    fetch('/api/', {method: 'GET'})
        .then(currentItems => currentItems.json())
        .then(currentItems => {
            data = currentItems.value.map((raw: RawItem) => new Item(raw));
            itemList.innerHTML = "";
            data.forEach(item => {
                itemList.appendChild(createEntryFromItem(item));
            });
            console.log(data);
        })
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
        .then(refreshItems)
        .then(() => {
            inputMemo.value = "";
            inputEventDate.value = "";
            composeOverlay.style.display = "none";
        });
}
