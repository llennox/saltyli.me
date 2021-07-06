
export function renderProfile(appState) {

    const content = document.querySelector('content');
    content.innerHTML = `
    <div class="d-flex justify-content-end">
        <ul class="list-group p-5">
            <li class="list-group-item"><button type="button" id="logOut" class="btn btn-primary me-2">Logout</button></li>
            <li class="list-group-item">A second item</li>
            <li class="list-group-item">A third item</li>
            <li class="list-group-item">A fourth item</li>
            <li class="list-group-item">And a fifth one</li>
        </ul>
    </div>
    `

    const logOut = document.getElementById("logOut");
    logOut.onclick = () => {
    appState.setCore('token', false)
    document.cookie = "token= ; expires = Thu, 01 Jan 1970 00:00:00 GMT";
    window.location.replace('#');
    };

}