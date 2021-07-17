import { renderUserContent } from './user-content.js';
import { makeid } from './utilities/utils.js';

export function renderSideBar(appState) {
    const navBar = document.querySelector('nav-bar');
    navBar.innerHTML = /*html*/`
    <svg xmlns="http://www.w3.org/2000/svg" style="display: none;">
      <symbol id="arrow-left" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M15 2a1 1 0 0 0-1-1H2a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V2zM0 2a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V2zm11.5 5.5a.5.5 0 0 1 0 1H5.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L5.707 7.5H11.5z"/>
      </symbol>
      <symbol id="menu" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5zm0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5z"/>
      </symbol>
      <symbol id="arrow-right" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M15 2a1 1 0 0 0-1-1H2a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V2zM0 2a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V2zm4.5 5.5a.5.5 0 0 0 0 1h5.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3a.5.5 0 0 0 0-.708l-3-3a.5.5 0 1 0-.708.708L10.293 7.5H4.5z"/>
      </symbol>
      <symbol id="people-circle" viewBox="0 0 16 16">
        <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z"/>
        <path fill-rule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"/>
      </symbol>
      <symbol id="conditions" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M1 2.5A1.5 1.5 0 0 1 2.5 1h1A1.5 1.5 0 0 1 5 2.5h4.134a1 1 0 1 1 0 1h-2.01c.18.18.34.381.484.605.638.992.892 2.354.892 3.895 0 1.993.257 3.092.713 3.7.356.476.895.721 1.787.784A1.5 1.5 0 0 1 12.5 11h1a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1-1.5 1.5h-1a1.5 1.5 0 0 1-1.5-1.5H6.866a1 1 0 1 1 0-1h1.711a2.839 2.839 0 0 1-.165-.2C7.743 11.407 7.5 10.007 7.5 8c0-1.46-.246-2.597-.733-3.355-.39-.605-.952-1-1.767-1.112A1.5 1.5 0 0 1 3.5 5h-1A1.5 1.5 0 0 1 1 3.5v-1zM2.5 2a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1zm10 10a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1z"/>
      </symbol>
      <symbol id="outlet" viewBox="0 0 16 16">
        <path d="M3.34 2.994c.275-.338.68-.494 1.074-.494h7.172c.393 0 .798.156 1.074.494.578.708 1.84 2.534 1.84 5.006 0 2.472-1.262 4.297-1.84 5.006-.276.338-.68.494-1.074.494H4.414c-.394 0-.799-.156-1.074-.494C2.762 12.297 1.5 10.472 1.5 8c0-2.472 1.262-4.297 1.84-5.006zm1.074.506a.376.376 0 0 0-.299.126C3.599 4.259 2.5 5.863 2.5 8c0 2.137 1.099 3.74 1.615 4.374.06.073.163.126.3.126h7.17c.137 0 .24-.053.3-.126.516-.633 1.615-2.237 1.615-4.374 0-2.137-1.099-3.74-1.615-4.374a.376.376 0 0 0-.3-.126h-7.17z"/>
        <path d="M6 5.5a.5.5 0 0 1 .5.5v1.5a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm4 0a.5.5 0 0 1 .5.5v1.5a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zM7 10v1h2v-1a1 1 0 0 0-2 0z"/>
      </symbol>
      <symbol id="graphs" viewBox="0 0 16 16">
        <path fill-rule="evenodd" d="M0 0h1v15h15v1H0V0zm10 3.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-1 0V4.9l-3.613 4.417a.5.5 0 0 1-.74.037L7.06 6.767l-3.656 5.027a.5.5 0 0 1-.808-.588l4-5.5a.5.5 0 0 1 .758-.06l2.609 2.61L13.445 4H10.5a.5.5 0 0 1-.5-.5z"/>
      </symbol>
    </svg>

      <div class="d-flex flex-column flex-shrink-0 p-3 bg-light offcanvas offcanvas-start" 
        id="offCanvasExample" aria-labelledby="offCanvasExampleLabel" tabindex="-1" data-bs-scroll="true" data-bs-backdrop="false" style="width: 280px; height: 100vh;">
       <div class="d-flex flex-row offcanvas-header">
        <a href="/" class="mb-3 mb-md-0 me-md-auto link-dark text-decoration-none">
          <img class="bi me-2" src="/static/img/lime1.svg" alt="" width="40" height="32">
          <span class="fs-4">Saltyli.me</span>
        </a>
        <button type="button" id="collapseButton" data-bs-dismiss="offcanvas" 
          data-bs-target="#offCanvasExample" aria-controls="offCanvasExample" class="btn me-2">
         <svg class="bi me-2 arrows" width="40" height="32"><use xlink:href="#arrow-left"/></svg>
        </button>
       </div>

          
        <hr>
          <ul class="nav nav-pills flex-column mb-auto">
            <li class="nav-item">
              <button type="button" id="graphsButton" class="nav-link link-dark active" aria-current="page">
                <svg class="bi me-2" width="16" height="16"><use xlink:href="#graphs"/></svg>
                Graphs
              </button>
            </li>
            <li>
              <button type="button" id="outletControlButton" class="nav-link link-dark">
                <svg class="bi me-2" width="16" height="16"><use xlink:href="#outlet"/></svg>
                Outlet Control 
              </button>
            </li>
            <li>
              <button type="button" id="conditionsButton" class="nav-link link-dark">
                <svg class="bi me-2" width="16" height="16"><use xlink:href="#conditions"/></svg>
                Conditions
              </button>
            </li>
            <li>
              <button type="button" id="profileButton" class="nav-link link-dark">
                <svg class="bi me-2" width="16" height="16"><use xlink:href="#people-circle"/></svg>
                Profile
              </button>
            </li>
          </ul>
          <hr>
      </div>
      <div class="d-flex p-2 bd-highlight">
        <div class="flex-fill">
          <button type="button" id="collapseButton" data-bs-toggle="offcanvas" 
            data-bs-target="#offCanvasExample" aria-controls="offCanvasExample" class="btn me-2">
          <svg class="bi me-2 arrows" width="40" height="32"><use xlink:href="#menu"/></svg>
          </button>
        </div>
    </div>
    `;
    const navLinks = document.querySelectorAll('.nav-link')
    navLinks.forEach(element => {
        if (appState.getCore()?.isMobile) {
          element.setAttribute("data-bs-dismiss", "offcanvas")
        }
        element.addEventListener('auxclick', function(e) {navigateState(e, appState) });
        element.addEventListener('click', function(e) {navigateState(e, appState) });
    });
    renderUserContent(appState);
};

function navigateState(e, appState) {
  if (e.button === 1) { //middle click
    window.open(window.location.href, makeid(10));
    return
  }
  const el = e.target || e.srcElement;
  const buttonId = el.id;

  const navLinks = document.querySelectorAll('.nav-link')
  navLinks.forEach(element => {
      if (element.id !== el.id) {
        element.classList.remove('active');
      }
  });
  const newPath = buttonId.replace("Button", "");
  el.classList.add('active');
  appState.setCore('path', newPath);
  renderUserContent(appState);
};
