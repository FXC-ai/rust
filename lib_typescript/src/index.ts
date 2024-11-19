import * as d3 from "d3";

function MakeKeys()
{

  console.log("MakeKeys");
  var ds1 = [["Mary", 1],["Jane", 4],["Anne", 2]];
  var ds2 = [["Anne", 5],["Jane", 3]];

  var scX = d3.scaleLinear().domain([1,5]).range([100,500]);
  var scY = d3.scaleLinear().domain([0,2]).range([100,500]);

  var j = -1;
  var k = -1;

  var svg = d3.select("#graph0");

  svg.selectAll("text").data(ds1, d => d[0]).enter().append("text").attr("x", 20).attr("y", d=>scY(++j)).text(d => d[0]);

  // svg.selectAll("circle").data(ds1).enter().append("circle").attr("r", 5).attr("fill", "red").arr("cx" ,d => scX(d[1])).attr("cy", d=>scY(++k)-5);


}


function makeUpdate()
{
  var ds1 = [[1,1, "green"],[1,5, "red"],[5,1, "yellow"],[5,5, "blue"]]
  var ds2 = [[1,3, "green"],[3,5, "red"],[5,3, "yellow"],[3,1, "blue"]]

  var scX = d3.scaleLinear().domain([1,5]).range([100,500]);
  var scY = d3.scaleLinear().domain([1,5]).range([100,500]);

  var svg = d3.select("#graph");

  svg.on("click", function(){
    [ds1, ds2] = [ds2, ds1];

    // 
    var cs = svg.selectAll("circle"). data(ds1, d => d[2]);

    cs.exit().remove();

    cs = cs.enter().append("circle").attr("r", 5).attr("fill", d => d[2]).merge(cs);

    cs.attr("cx", d=>scX(d[0])).attr("cy", d=>scY(d[1]));
  });


}


function makePieLoad(parent : HTMLElement, data : [number, number], color_task_done : String, color_task_to_do : String)  : {makeUpdate}
{

  let width = parent.getBoundingClientRect().width;
  let height = parent.getBoundingClientRect().height;

  // console.log(width, " ", height);

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


let elem = document.getElementById("animated_pie") as HTMLElement;

const chart = makePieLoad(elem, [0, 10],"#FFDC00", "#0074D9");

let intervalId = setInterval(() => {
  try {
    chart.makeUpdate();

  }
  catch
  {

    clearInterval(intervalId);
  }
}, 500);








makeUpdate();
MakeKeys();
// makePie("#animated_pie");