import * as d3 from "d3";


function makePieLoad(parent : HTMLElement, data : [number, number], color_task_done : String, color_task_to_do : String)  : {makeUpdate}
{
  let width = parent.getBoundingClientRect().width;
  let height = parent.getBoundingClientRect().height;
  let radius = Math.min(width, height) / 2;

  let nbr_task_done = data[0];
  const nbr_task_to_do = data[1];

  var svg = d3.select(parent)
    .append("svg")
    .attr("width", width)
    .attr("height", height)
    .append("g")
    .attr("transform", `translate(${width / 2}, ${height / 2})`);

  const pie = d3.pie()
    .value(d => d)
    .sort(null);

  const arc = d3.arc().innerRadius(0).outerRadius(radius);

  const color = d3.scaleOrdinal().range([color_task_done, color_task_to_do]);

  const paths = svg.selectAll("path")
    .data(pie([nbr_task_done, nbr_task_to_do - nbr_task_done]))
    .enter()
    .append("path")
    .attr("d", arc)
    .attr("fill", i => color(i))
    .each(function(d) {
      this._current = d;
    });

  function updatePie() : void 
  {
    const newData = [nbr_task_done, nbr_task_to_do - nbr_task_done];
    const updatedArcs = pie(newData);

    paths
      .data(updatedArcs)
      .transition()
      .duration(1000)
      .attrTween("d", function(d) {
        const interpolate = d3.interpolate(this._current, d);
        this._current = interpolate(1);
        return function(t) {
          return arc(interpolate(t));
        };
      });
  }

  function makeUpdate() : void {
    if (nbr_task_done < nbr_task_to_do) {
      nbr_task_done += 1;
      updatePie();
    }
    else
    {
      throw new Error("Task are all completed.")
      console.log("Toutes les tâches sont terminées !");
    }
  }

  return  {makeUpdate};
}


// let elem = document.getElementById("animated_pie") as HTMLElement;

// const chart = makePieLoad(elem, [0, 10],"#FFDC00", "#0074D9");

// let intervalId = setInterval(() => {
//   try {
//     chart.makeUpdate();

//   }
//   catch
//   {
//     clearInterval(intervalId);
//   }
// }, 500);