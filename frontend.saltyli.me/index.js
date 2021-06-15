import { renderNavBar } from './nav-bar.js';
import { renderHome } from './home.js';
import { renderLogin } from './login.js';

window.addEventListener('load', () => {
    //registerSW();
    const navBar = document.createElement('nav-bar');
    const content = document.createElement('content');
    const main = document.querySelector('main');
    main.appendChild(navBar);
    main.appendChild(content);
    render(document.location.hash)
    window.addEventListener('popstate', function(){
        render(document.location.hash);
    });
});

function render(hashUrl) {
    let url = hashUrl.replace('#', '');
    const urlList = url.split('/');
    switch (urlList[0]) {
        case 'login':
            renderNavBar(urlList[0]);
            renderLogin();
            break;
        case 'register':
            renderNavBar(urlList[0]);
            renderLogin();
            break;
        default:
            renderNavBar(urlList[0]);
            renderHome();
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
