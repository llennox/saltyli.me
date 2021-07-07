export function renderHome() {
    const content = document.querySelector('content');
    content.innerHTML = /*html*/`
    <div class="container pt-5">
    <div class="row">
        <div class="card pt-3 pb-3">
        <p>Welcome to saltylime</p><br>
	    <p> contact me at <a href="#" class="cryptedmail"
   	        data-name="gonnellcough"
   	        data-domain="gmail"
   	        data-tld="com"
   	        onclick="window.location.href = 'mailto:' + this.dataset.name + '@' + this.dataset.domain + '.' + this.dataset.tld; return false;"></a> if you have questions</p>
        </div>
    </div>
    </div>
    `;
}
