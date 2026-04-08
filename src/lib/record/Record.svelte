<script>
  let status = $state('idle');

  function startRecording() {
    if (status === 'idle' || status === 'stopped') status = 'recording';
    else if (status === 'recording') status = 'stopped';
  }
</script>

<button
  onclick={startRecording}
  class="button"
  title="Record"
></button>
<!--<p>Status: {status}</p>-->

<style>
  .button {
    width: 15px;
    height: 15px;
    border-radius: 50%;
    border: none;
    background-color: darkred;
    cursor: pointer;
  }
</style>
