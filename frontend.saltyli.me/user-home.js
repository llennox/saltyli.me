import './static/bootstrap/js/bootstrap.bundle.min.js';
export function renderUserHome(urlList, appState) {
    const content = document.querySelector('content');
    content.innerHTML = `
<svg xmlns="http://www.w3.org/2000/svg" style="display: none;">
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

<main>
  <div class="d-inline-flex justify-between">
    <div class="d-flex flex-column justify-content-start flex-shrink-0" style="width: 4.5rem;">
      <ul class="nav nav-pills nav-flush flex-column mb-auto text-center">
        <li>
          <a href="#graphs" class="nav-link link-dark py-3 border" title="Graphs" data-bs-toggle="tooltip" data-bs-placement="right">
            <svg class="bi" width="24" height="24"><use xlink:href="#graphs"/></svg>
          </a>
        </li>
        <li>
          <a href="#" class="nav-link link-dark py-3 border" title="Outlet Control" data-bs-toggle="tooltip" data-bs-placement="right">
            <svg class="bi" width="24" height="24"><use xlink:href="#outlet"/></svg>
          </a>
        </li>
        <li>
          <a href="#" class="nav-link link-dark py-3 border" title="Conditions" data-bs-toggle="tooltip" data-bs-placement="right">
            <svg class="bi" width="24" height="24"><use xlink:href="#conditions"/></svg>
          </a>
        </li>
        <li>
          <a href="#" class="nav-link link-dark py-3 border" title="Profile" data-bs-toggle="tooltip" data-bs-placement="right">
            <svg class="bi" width="24" height="24"><use xlink:href="#people-circle"/></svg>
          </a>
        </li>
      </ul>
    </div>
    <div class="d-flex flex-column p-4">
      <div id="firstGraph"> main content </div>
      <h1> main content2 </h1>
    </div>
  </div>
</main>
    `;
  const tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
  tooltipTriggerList.forEach(function (tooltipTriggerEl) {
    const tooltip = new bootstrap.Tooltip(tooltipTriggerEl);
    tooltipTriggerEl.addEventListener('click', () => tooltip.hide());
  });

// Plotly code
  Plotly.d3.csv("https://raw.githubusercontent.com/plotly/datasets/master/finance-charts-apple.csv", function(err, rows){

  function unpack(rows, key) {
  return rows.map(function(row) { return row[key]; });
}


var trace1 = {
  type: "scatter",
  mode: "lines",
  name: 'AAPL High',
  x: unpack(rows, 'Date'),
  y: unpack(rows, 'AAPL.High'),
  line: {color: '#17BECF'}
}

var data = [trace1];

var layout = {
  title: 'Time Series with Rangeslider',
  xaxis: {
    autorange: true,
    range: ['2015-02-17', '2017-02-16'],
    rangeselector: {buttons: [
        {
          count: 1,
          label: '1m',
          step: 'month',
          stepmode: 'backward'
        },
        {
          count: 6,
          label: '6m',
          step: 'month',
          stepmode: 'backward'
        },
        {step: 'all'}
      ]},
    rangeslider: {range: ['2015-02-17', '2017-02-16']},
    type: 'date'
  },
  yaxis: {
    autorange: true,
    range: [86.8700008333, 138.870004167],
    type: 'linear'
  }
};

Plotly.newPlot('firstGraph', data, layout);
})

}
