<script>
  import { onMount } from "svelte";

  let time_array_go = "loading...";
  let time_array_js = "loading...";

  let time_matrix_go = "loading...";
  let time_matrix_js = "loading...";

  let time_json_go = "loading...";
  let time_json_js = "loading...";

  onMount(async () => {
    const res_array_go = await fetch("/api/array");
    console.log(res_array_go);
    time_array_go = await res_array_go.text();

    const res_array_js = await fetch("/array_js");
    console.log(res_array_js);
    time_array_js = await res_array_js.text();

    const res_matrix_go = await fetch("/api/matrix");
    console.log(res_matrix_go);
    time_matrix_go = await res_matrix_go.text();

    const res_matrix_js = await fetch("/matrix_js");
    console.log(res_matrix_js);
    time_matrix_js = await res_matrix_js.text();

    const res_json_go = await fetch("/api/json");
    console.log(res_json_go);
    time_json_go = await res_json_go.text();

    const res_json_js = await fetch("/json_js");
    console.log(res_json_js);
    time_json_js = await res_json_js.text();
  });
</script>

<h1>Time to add 5000 elements to an array</h1>
<h2>Go: {time_array_go}</h2>
<h2>Javascript: {time_array_js}</h2>
<br />
<h1>Time to multiply 2 300x300 matrices</h1>
<h2>Go: {time_matrix_go}</h2>
<h2>Javascript: {time_matrix_js}</h2>
<br />
<h1>Parse JSON string</h1>
<h2>Go: {time_json_go}</h2>
<h2>Javascript: {time_json_js}</h2>
