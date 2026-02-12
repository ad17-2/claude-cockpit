<script lang="ts">
  import { page } from "$app/state";
</script>

<div class="flex h-screen items-center justify-center bg-bg-primary p-8">
  <div class="max-w-lg space-y-4">
    <h1 class="text-sm font-medium text-text-primary">// error {page.status}</h1>
    <pre class="overflow-auto border border-border-primary bg-bg-tertiary p-4 text-xs text-danger">{page.error?.message}</pre>
    <a href="/" class="text-xs text-accent hover:text-accent-hover">> back to dashboard</a>
  </div>
</div>
