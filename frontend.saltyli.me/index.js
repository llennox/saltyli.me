import { renderNavBar } from './nav-bar.js';
import { renderUserHome } from './user-home.js'
import { renderSideBar } from './side-bar.js';
import { renderHome } from './home.js';
import { renderLogin } from './login.js';
import AppState from './app-state.js';

window.addEventListener('load', () => {
    let appState = new AppState();
    let _jwt = getCookie('token');
    // if user has cookie saved
    if (_jwt) {
      appState.setCore('token', _jwt)
    };
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

function getCookie(name) {
  function escape(s) { return s.replace(/([.*+?\^$(){}|\[\]\/\\])/g, '\\$1'); }
  var match = document.cookie.match(RegExp('(?:^|;\\s*)' + escape(name) + '=([^;]*)'));
  return match ? match[1] : null;
}

function render(hashUrl, appState) {
    let url = hashUrl.replace('#', '');
    const urlList = url.split('/');
    console.log(appState.getCore());
    console.log(getCookie('token'));

    if (!appState.getCore()?.token) {
      switch (urlList[0]) {
          case 'login':
              renderNavBar(urlList[0], appState);
              renderLogin(appState);
              break;
          case 'register':
              renderNavBar(urlList[0], appState);
              renderLogin();
              break;
          default:
              renderNavBar(urlList[0], appState);
              renderHome(appState);
              break;
      }
    } else {
      console.log("User logged in")
      document.querySelector('nav-bar').innerHTML = ''; 
      switch (appState.getCore()?.path) {
          case 'graphs':
              renderSideBar(appState);
              //renderGraphs(appState);
              break;
          case 'outlet-control':
              renderSideBar(appState);
              //renderOutletControl();
              break;
          case 'conditions':
              renderSideBar(appState);
              //renderOutletControl();
              break;
          case 'profile':
              renderSideBar(appState);
              //renderOutletControl();
              break;
          default:
              renderSideBar(appState);
              //renderUserHome(urlList, appState);
              break;
      }

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
