<script>
  import { onMount } from "svelte";

  let time_array_go = "loading...";
  let time_array_rust = "loading...";
  let time_array_js = "loading...";

  let time_matrix_go = "loading...";
  let time_matrix_js = "loading...";

  let time_json_go = "loading...";
  let time_json_js = "loading...";

  let time_sort_go = "loading...";
  let time_sort_js = "loading...";

  let time_pi_go = "loading...";
  let time_pi_js = "loading...";

  onMount(() => {
    fetch("/api/array")
      .then((res) => res.text())
      .then((text) => (time_array_go = text));

    fetch("/api/rust/array")
      .then((res) => res.text())
      .then((text) => (time_array_go = text));

    fetch("/array_js")
      .then((res) => res.text())
      .then((text) => (time_array_js = text));

    fetch("/api/matrix")
      .then((res) => res.text())
      .then((text) => (time_matrix_go = text));

    fetch("/matrix_js")
      .then((res) => res.text())
      .then((text) => (time_matrix_js = text));

    fetch("/api/json")
      .then((res) => res.text())
      .then((text) => (time_json_go = text));

    fetch("/json_js")
      .then((res) => res.text())
      .then((text) => (time_json_js = text));

    fetch("/api/sort")
      .then((res) => res.text())
      .then((text) => (time_sort_go = text));

    fetch("/sort_js")
      .then((res) => res.text())
      .then((text) => (time_sort_js = text));

    fetch("/api/pi")
      .then((res) => res.text())
      .then((text) => (time_pi_go = text));

    fetch("/pi_js")
      .then((res) => res.text())
      .then((text) => (time_pi_js = text));
  });
</script>

<h1>Time to add 5000 elements to an array</h1>
<h2>Go: {time_array_go}</h2>
<h2>Rust: {time_array_rust}</h2>
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
<br />
<h1>Monte Carlo Pi Estimation</h1>
<h2>Go: {time_pi_go}</h2>
<h2>Javascript: {time_pi_js}</h2>
