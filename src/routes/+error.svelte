<script lang="ts">
  import { page } from "$app/state";
</script>

<div class="flex h-screen items-center justify-center bg-bg-primary p-8">
  <div class="max-w-lg space-y-4">
    <h1 class="text-lg font-semibold text-text-primary">{page.status} Error</h1>
    <pre class="overflow-auto rounded-md border border-border-primary bg-bg-tertiary p-4 font-mono text-xs text-danger">{page.error?.message}</pre>
    <a href="/" class="text-xs text-accent hover:underline">Back to Dashboard</a>
  </div>
</div>
