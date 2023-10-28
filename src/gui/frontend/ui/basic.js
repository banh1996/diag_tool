const vendorIdToName = {
    common: 'common',
    volvo: 'volvo',
    vinfast: 'vinfast',
};

const doipversionIdToName = {
    ISO13400_2: 'ISO13400_2',
    ISO13400_3: 'ISO13400_3',
};

var light = document.querySelector('.light');
              
const ipaddressInput = document.querySelector('#ipaddress-txt')
const remoteportInput = document.querySelector('#remoteport-txt')
var connectBtn = document.querySelector('#connect-btn');
var isConnected = false;

const sendudsBtn = document.querySelector('#senduds-btn')
const diagInput = document.querySelector('#diag-cmd')
const senddoipBtn = document.querySelector('#senddoip-btn')
const doipInput = document.querySelector('#doip-cmd')
const activationInput = document.querySelector('#activation-txt')

const logBox = document.querySelector("#log-box");


function updateResponse(response) {
    logBox.append( typeof response === 'string' ? response : JSON.stringify(response))
    logBox.append('\n');
}


sendudsBtn.addEventListener('click', () => {
window.__TAURI__
    .invoke('senduds', {
        value: diagInput.value
    })
    .then(updateResponse)
    .catch(updateResponse)
})

senddoipBtn.addEventListener('click', () => {
window.__TAURI__
    .invoke('senddoip', {
        value: doipInput.value
    })
    .then(updateResponse)
    .catch(updateResponse)
})




connectBtn.addEventListener('click', function() {
    if (!isConnected) {
      connect();
    } else {
      disconnect();
    }
});


function connect() {
    window.__TAURI__
        .invoke('connect', {
            remoteip: ipaddressInput.value,
            port: remoteportInput.value,
            role: "client",
            vendor: vendorIdToName[vendorSelect.value],
            doipversion: doipversionIdToName[doipversionSelect.value],
            testeraddr: TesteraddrInput.value,
            ecuaddr: ECUaddrInput.value,
            sgaaddr: SGAaddrInput.value,
            activationcode: activationInput.value,
        })
        .then(function(response) {
        isConnected = true;
        light.classList.add('green');
        connectBtn.textContent = 'DISCONNECT';
        console.log('Connected:', response);
    })
    .catch(function(error) {
        console.log('Connection error:', error);
    });
}
  
function disconnect() {
    window.__TAURI__
        .invoke('disconnect')
        .then(function(response) {
        isConnected = false;
        light.classList.remove('green');
        connectBtn.textContent = 'CONNECT';
        console.log('Disconnected:', response);
    })
    .catch(function(error) {
        console.log('Disconnection error:', error);
    });
}
