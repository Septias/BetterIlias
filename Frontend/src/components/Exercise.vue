
<template>
  <Leaf color="text-purple-500" :node="node">
    <template #default>
      <carbon-settings></carbon-settings>
    </template>
    <template #body>
      <a
        v-if="!edit_visibility"
        :href="'https://ilias.uni-freiburg.de/' + node.uri"
        >{{ node.title }}</a
      >
      <template v-else>{{ node.title }}</template>
    </template>
  </Leaf>
</template>

<script>
import { useVisibility } from "./compositions";
export default {
  name: "Exercise",
  props: {
    node: {
      type: Object,
      required: true,
    },
  },
  setup() {
    const { edit_visibility } = useVisibility();
    return { edit_visibility };
  },
};
</script>