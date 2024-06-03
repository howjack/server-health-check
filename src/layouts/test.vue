<template>
  <main class="h-screen w-screen">

    <UiResizablePanelGroup direction="horizontal" class="h-full items-stretch">

      <UiResizablePanel :default-size="5" :max-size="20" :min-size="10" :collapsible="isCollapsible" :collapsed-size="2" class="min-w-14 max-w-sm"
        @collapse="isCollapsible = true" @expand="isCollapsible = false">
        <TestNav :isCollapsed="isCollapsible" :links="links" />
      </UiResizablePanel>

      <UiResizableHandle with-handle />

      <UiResizablePanel class="overflow-scroll">
        <!-- TODO: Resolver scroll -->
        <slot />
      </UiResizablePanel>

    </UiResizablePanelGroup>
  </main>
</template>

<script setup>

const isCollapsible = ref(true)

const links = [
  {
    title: 'APIs',
    route: '/test/apis',
    icon: 'lucide:link',
  },
  {
    title: 'Monitoring',
    route: '/test',
    icon: 'lucide:bar-chart',
  }
]

</script>