<script>
  import { onMount } from "svelte";

  let time_go = "loading...";
  let time_js = "loading...";
  onMount(async () => {
    const res_go = await fetch("/api/array");
    console.log(res_go);
    time_go = await res_go.text();

    const res_js = await fetch("/array_js");
    console.log(res_js);
    time_js = await res_js.text();
  });
</script>

<h1>Time to add 5000 elements to an array</h1>
<h2>Go: {time_go}</h2>
<h2>Javascript: {time_js}</h2>
