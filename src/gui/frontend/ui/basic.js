const vendorIdToName = {
    common: 'common',
    volvo: 'volvo',
    vinfast: 'vinfast',
};

const doipversionIdToName = {
    ISO13400_2: 'ISO13400_2',
    ISO13400_3: 'ISO13400_3',
};


const logBox = document.querySelector("#log-box");
function updateResponse(response) {
    logBox.append( typeof response === 'string' ? response : JSON.stringify(response))
    logBox.append('\n');
}


// Handle send diag events
const sendudsBtn = document.querySelector('#senduds-btn')
const diagInput = document.querySelector('#diag-cmd')
sendudsBtn.addEventListener('click', () => {
window.__TAURI__
    .invoke('senduds', {
        value: diagInput.value
    })
    .then(updateResponse)
    .catch(updateResponse)
})


// Handle send doip events
const senddoipBtn = document.querySelector('#senddoip-btn')
const doipInput = document.querySelector('#doip-cmd')
senddoipBtn.addEventListener('click', () => {
window.__TAURI__
    .invoke('senddoip', {
        value: doipInput.value
    })
    .then(updateResponse)
    .catch(updateResponse)
})


// Handle connect/disconnect ECU
var light = document.querySelector('.light');      
const ipaddressInput = document.querySelector('#ipaddress-txt')
const remoteportInput = document.querySelector('#remoteport-txt')
var connectBtn = document.querySelector('#connect-btn');
var isConnected = false;
const activationInput = document.querySelector('#activation-txt')
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
            testerpresentenable: testerpresentcheckbox.checked,
            testerpresentinterval: TesterpresentIntervalInput.value,
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


//Import config parameters
const reader = new FileReader();
fileconfigInput.addEventListener('change', function(event) {
    const file = event.target.files[0];
    reader.onload = function() {
        const configData = JSON.parse(reader.result);
        // Update the values of the other input elements.
        document.getElementById('ipaddress-txt').value = configData.ethernet.remote_ip;
        document.getElementById('remoteport-txt').value = configData.ethernet.remote_port;
        document.getElementById('vendor').value = configData.ethernet.vendor;
        document.getElementById('Testeraddr-txt').value = configData.doip.tester_addr;
        document.getElementById('ECUaddr-txt').value = configData.doip.ecu_addr;
        document.getElementById('SGAaddr-txt').value = configData.doip.sga_addr;
        document.getElementById('activation-txt').value = configData.doip.activation_code;
        if (configData.doip.version == "0x2") {
            document.getElementById('doipversion').value = "ISO13400_2";
        }
        document.getElementById('TesterpresentInterval-txt').value = configData.parameter.tester_present_interval;
        document.getElementById('testerpresent-checkbox').checked = configData.parameter.tester_present;

        window.__TAURI__.invoke('updateconfig', {
            config: configData
        })
    };
    reader.readAsText(file);

    // Reset the value of the file input element
    fileconfigInput.value = null;
});

//Handle SWDL events
flashBtn.addEventListener('click', () => {
    window.__TAURI__
        .invoke('flash')
        .then(updateResponse)
        .catch(updateResponse)
})
fileswdlInput.addEventListener('click', () => {
    window.__TAURI__
        .invoke('selectswdlfiles')
})

//Handle sequence events
executeBtn.addEventListener('click', () => {
    window.__TAURI__
        .invoke('executesequence')
        .then(updateResponse)
        .catch(updateResponse)
})
filesequenceInput.addEventListener('click', () => {
    window.__TAURI__
        .invoke('selectsequencefile')
})

//Handle Security-Access events
sendSABtn.addEventListener('click', () => {
    window.__TAURI__
        .invoke('sendsecurityaccess', {
            level: SAlevelInput.value,
            key: SAkeyInput.value,
        })
        .then(updateResponse)
        .catch(updateResponse)
})

//Handle Tester-Present events
testerpresentcheckbox.addEventListener('change', function(event) {
    window.__TAURI__
        .invoke('triggertesterpresent', {
            enable: testerpresentcheckbox.checked,
            interval: TesterpresentIntervalInput.value,
        })
        .then(updateResponse)
        .catch(updateResponse)
})