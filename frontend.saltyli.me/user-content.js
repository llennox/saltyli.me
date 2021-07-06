import { renderProfile } from './profile.js';
import { renderGraphs } from './user-graphs.js';

export function renderUserContent(appState) {

    switch (appState.getCore()?.path) {
        case 'graphs':
            renderGraphs(appState);
            break;
        case 'outletControl':
            //renderOutletControl();
            break;
        case 'conditions':
            //renderConditions();
            break;
        case 'profile':
            renderProfile(appState);
            break;

            

    }

        //renderUserHome(urlList, appState);
}