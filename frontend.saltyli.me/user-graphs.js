import './static/bootstrap/js/bootstrap.bundle.min.js';
import './static/d3.min.js';
export function renderGraphs(appState) {
    const content = document.querySelector('content');
    content.innerHTML = `
      <div class="d-flex flex-column justify-content-center p-4">
        <div id="firstGraph"></div>
      </div>
    `;
    //renderRangeSlider();
    renderRangeSlider1();
}

function renderRangeSlider() {
  var rawDataURL = 'https://raw.githubusercontent.com/plotly/datasets/master/2016-weather-data-seattle.csv';
  var xField = 'Date';
  var yField = 'Mean_TemperatureC';

  var selectorOptions = {
      buttons: [{
          step: 'month',
          stepmode: 'backward',
          count: 1,
          label: '1m'
      }, {
          step: 'month',
          stepmode: 'backward',
          count: 6,
          label: '6m'
      }, {
          step: 'year',
          stepmode: 'todate',
          count: 1,
          label: 'YTD'
      }, {
          step: 'year',
          stepmode: 'backward',
          count: 1,
          label: '1y'
      }, {
          step: 'all',
      }],
  };

  d3.csv(rawDataURL).then( (rawData) => {
      var data = prepData(rawData);
      var layout = {
          title: 'Time series with range slider and selectors',
          xaxis: {
              rangeselector: selectorOptions,
              rangeslider: {}
          },
          yaxis: {
              fixedrange: true
          }
      };
      Plotly.newPlot('firstGraph', data, layout);
    })
      //if (err) console.log(err);
      //if(err) throw err;



  function prepData(rawData) {
      var x = [];
      var y = [];

      rawData.forEach(function(datum, i) {

          x.push(new Date(datum[xField]));
          y.push(datum[yField]);
      });

      return [{
          mode: 'lines',
          x: x,
          y: y
      }];
  }
}

function renderRangeSlider1() {
  var rawDataURL = 'https://raw.githubusercontent.com/plotly/datasets/master/2016-weather-data-seattle.csv';
  var xField = 'Date';
  var yField = 'Mean_TemperatureC';

  var selectorOptions = {
      buttons: [{
          step: 'month',
          stepmode: 'backward',
          count: 1,
          label: '1m'
      }, {
          step: 'month',
          stepmode: 'backward',
          count: 6,
          label: '6m'
      }, {
          step: 'year',
          stepmode: 'todate',
          count: 1,
          label: 'YTD'
      }, {
          step: 'year',
          stepmode: 'backward',
          count: 1,
          label: '1y'
      }, {
          step: 'all',
      }],
  };

  d3.csv(rawDataURL).then( (rawData) => {
      var data = prepData(rawData);
      var layout = {
          title: 'Time series with range slider and selectors',
          xaxis: {
              rangeselector: selectorOptions,
              rangeslider: {}
          },
          yaxis: {
              fixedrange: true
          }
      };
      Plotly.newPlot('firstGraph', data, layout);
    })
      //if (err) console.log(err);
      //if(err) throw err;



  function prepData(rawData) {
      var x = [];
      var y = [];

      rawData.forEach(function(datum, i) {

          x.push(new Date(datum[xField]));
          y.push(datum[yField]);
      });

      return [{
          mode: 'lines',
          x: x,
          y: y
      }];
  }
}