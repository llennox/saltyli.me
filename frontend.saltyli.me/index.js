import { renderNavBar } from './nav-bar.js';
import { renderHome } from './home.js';
import { renderLogin } from './login.js';
import AppState from './app-state.js';

window.addEventListener('load', () => {
    //registerSW();
    let appState = new AppState();
    // if cookie exists setCore in appState here
    const navBar = document.createElement('nav-bar');
    const content = document.createElement('content');
    const main = document.querySelector('main');
    main.appendChild(navBar);
    main.appendChild(content);
    render(document.location.hash, appState);
    window.addEventListener('popstate', function(){
        render(document.location.hash, appState);
    });
});

function render(hashUrl, appState) {
    let url = hashUrl.replace('#', '');
    const urlList = url.split('/');
    console.log(appState.getCore());
    console.log(document.cookie);
    switch (urlList[0]) {
        case 'login':
            renderNavBar(urlList[0]);
            renderLogin(appState);
            break;
        case 'register':
            renderNavBar(urlList[0]);
            renderLogin();
            break;
        default:
            renderNavBar(urlList[0]);
            renderHome(appState);
            break;
    }
}


async function registerSW() {
  if ('serviceWorker' in navigator) {
    try {
      await navigator.serviceWorker.register('./sw.js');
    } catch (e) {
      console.log(`SW registration failed`);
    }
  }
}
