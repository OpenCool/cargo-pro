<script setup>
import axios from "@/utils/axios";
import {ref, computed, reactive, onMounted} from 'vue'
import {TabGroup, TabList, Tab, TabPanels, TabPanel} from '@headlessui/vue'

let categories = ref(['README', 'Contributors', 'Stats', 'Dependencies'])
const readme = ref()


onMounted(()=>{
  axios.get("/profile/readme", {}
  ).then((resp)=>{
    console.log("resp:", resp);
    readme.value = resp.data.toString()
  })
  .catch((error)=>{
    console.log(error,'失敗');
  })
})
</script>

<template>

  <div class="container mx-auto">
    <BaseCard noPadding class="user-profile overflow-hidden relative">
      <div class="header-cover"></div>
      <div class="flex justify-center z-10">
        <div class="text-center">
          <p class="text-2xl">Cargo Pro</p>
          <p class="text-gray-600">Analyze rust projects like a Pro!</p>
        </div>
      </div>

      <div class="mt-10 p-5">

        <TabGroup>
          <TabList class="flex bg-white justify-center">
            <Tab
                v-for="category in categories"
                as="template"
                :key="category"
                v-slot="{ selected }"
            >
              <button
                  class="font-semibold border-b-2 border-white"
                  :class="[
                    'px-3 py-1 text-sm leading-5 ',
                    'focus:outline-none',
                    selected
                      ? 'text-primary border-b-2 border-primary'
                      : 'text-gray-600',
                  ]"
              >
                {{ category }}
              </button>
            </Tab>
          </TabList>

          <TabPanels class="mt-10">
            <TabPanel>
              <div>
                <v-md-preview :text="readme"></v-md-preview>
              </div>
            </TabPanel>

            <TabPanel>
              <div>
                Contributors
              </div>
            </TabPanel>

            <TabPanel>
              <div>
                Stats
              </div>
            </TabPanel>

            <TabPanel>
              <div>
                Dependencies
              </div>
            </TabPanel>
          </TabPanels>
        </TabGroup>
      </div>
    </BaseCard>
  </div>
</template>
<style lang="scss" scoped>
.user-profile {
  .header-cover {
    background-image: url("/images/gif/move.gif");
    position: relative;
    background-size: cover;
    background-repeat: no-repeat;
    height: 180px;
    width: 540px;
    margin: 5px auto;
  }
}

</style>
