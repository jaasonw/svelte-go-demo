<script>
  import { onMount } from "svelte";

  let time_array_go = "loading...";
  let time_array_js = "loading...";

  let time_matrix_go = "loading...";
  let time_matrix_js = "loading...";

  let time_json_go = "loading...";
  let time_json_js = "loading...";

  let time_sort_go = "loading...";
  let time_sort_js = "loading...";

  onMount(async () => {
    const res_array_go = await fetch("/api/array");
    time_array_go = await res_array_go.text();

    const res_array_js = await fetch("/array_js");
    time_array_js = await res_array_js.text();

    const res_matrix_go = await fetch("/api/matrix");
    time_matrix_go = await res_matrix_go.text();

    const res_matrix_js = await fetch("/matrix_js");
    time_matrix_js = await res_matrix_js.text();

    const res_json_go = await fetch("/api/json");
    time_json_go = await res_json_go.text();

    const res_json_js = await fetch("/json_js");
    time_json_js = await res_json_js.text();

    const res_sort_go = await fetch("/api/sort");
    time_sort_go = await res_sort_go.text();

    const res_sort_js = await fetch("/sort_js");
    time_sort_js = await res_sort_js.text();
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
<br />
<h1>Sort 5000000 elements</h1>
<h2>Go: {time_sort_go}</h2>
<h2>Javascript: {time_sort_js}</h2>
