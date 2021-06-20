export function renderNavBar(route) {
    const navBar = document.querySelector('nav-bar');

    navBar.innerHTML = `
      <div class="container">
        <header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-4 border-bottom background-blur">
          <div class="col-md-3">
          </div>

          <ul class="nav col-12 col-md-auto mb-2 justify-content-center align-items-center mb-md-0">
            <li><a href="#" class="nav-link px-2 link-secondary">Home</a></li>
            <li><a href="#" class="nav-link px-2 link-dark">Features</a></li>
            <li><a href="#" class="nav-link px-2 link-dark">Pricing</a></li>
            <li><a href="#" class="nav-link px-2 link-dark">FAQs</a></li>
            <li><a href="#" class="nav-link px-2 link-dark">About</a></li>
          </ul>

          <div class="col-md-3 text-end">
            <button type="button" id="navigateToLogin" class="btn btn-primary me-2">Login</button>
          </div>
        </header>
      </div>
    `;
    const loginLink = document.getElementById("navigateToLogin");
    loginLink.onclick = () => window.location.replace('#login');
}