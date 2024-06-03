<template>
  <main :data-collapsed="isCollapsed" class="group flex flex-col gap-4 py-2 data-[collapsed=true]:py-2">
    <nav class="space-y-2">

      <section class="grid gap-1 px-2 group-[[data-collapsed=true]]:justify-center group-[[data-collapsed=true]]:px-2">
        <UiButton variant="outline" :class="{ 'justify-start': !isCollapsed }" asChild
          :size="isCollapsed ? 'icon' : 'sm'">

          <NuxtLink href="/">
            <Icon name="lucide:arrow-left" size="16" :class="{ 'mr-2': !isCollapsed }" />
            <span v-if="!isCollapsed">Voltar para Inicio</span>
          </NuxtLink>
        </UiButton>
      </section>

      <UiSeparator />

      <section class="grid gap-1 px-2 group-[[data-collapsed=true]]:justify-center group-[[data-collapsed=true]]:px-2">
        <template v-for="(link, index) of links">

          <UiButton v-if="isCollapsed" size="icon" asChild
            :variant="route.fullPath === link.route ? 'secondary' : 'ghost'">

            <NuxtLink :key="`1-${index}`" :href="link.route">
              <Icon :name="link.icon" size="16" />
              <span class="sr-only">{{ link.title }}</span>
            </NuxtLink>
          </UiButton>



          <UiButton v-else size="sm" asChild class="justify-start"
            :variant="route.fullPath === link.route ? 'secondary' : 'ghost'">

            <NuxtLink :key="`2-${index}`" :href="link.route">
              <Icon :name="link.icon" size="16" class="mr-2" />
              {{ link.title }}
            </NuxtLink>
          </UiButton>
        </template>
      </section>
    </nav>
  </main>
</template>

<script setup>

defineProps(['links', 'isCollapsed'])

const route = useRoute()

console.log(route.fullPath)

</script>