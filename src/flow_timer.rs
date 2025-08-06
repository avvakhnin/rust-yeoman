use std::{
    collections::BTreeMap,
    task::{Context, Poll, Waker},
};

use edict::{
    flow::{FlowEntity, FlowWorld},
    prelude::{Res, ResMut},
    scheduler::Scheduler,
    world::World,
};

struct FlowTimerMap {
    current_time: f32,
    wakers: BTreeMap<i32, Vec<Waker>>,
}

impl FlowTimerMap {
    fn new() -> Self {
        FlowTimerMap {
            current_time: 0f32,
            wakers: BTreeMap::new(),
        }
    }
    fn wake_flows(&mut self, time_delta: f32) {
        self.current_time += time_delta;
        let time = self.current_time as i32;
        let mut not_ready = self.wakers.split_off(&time);
        std::mem::swap(&mut self.wakers, &mut not_ready);
        not_ready.into_values().flatten().for_each(|v| v.wake());
    }
    fn add_flow(&mut self, waker: Waker, release_time: i32) {
        if let Some(v) = self.wakers.get_mut(&release_time) {
            v.push(waker);
        } else {
            self.wakers.insert(release_time, vec![waker]);
        }
    }
    fn add_flow_context_f32(&mut self, context: &Context, release_time: f32) {
        self.add_flow(context.waker().clone(), release_time as i32)
    }
}

pub fn init_flow_timers(world: &mut World, scheduler: &mut Scheduler) {
    world.insert_resource(FlowTimerMap::new());
    scheduler.add_system(timer);
}

fn get_current_time(flow_world: &FlowWorld) -> f32 {
    flow_world.map(|w| w.expect_resource::<FlowTimerMap>().current_time)
}

fn timer(time_delta: Res<f32>, mut wmap: ResMut<FlowTimerMap>) {
    wmap.wake_flows(*time_delta);
}

pub async fn _wait_pause(fw: FlowWorld, pause_time: f32) {
    let now_time = get_current_time(&fw);
    _wait_until(fw, now_time + pause_time).await;
}

pub async fn _wait_until(fw: FlowWorld, release_time: f32) {
    fw.poll(move |w, cx| {
        let mut wmap = w.get_resource_mut::<FlowTimerMap>().unwrap();
        if release_time <= wmap.current_time {
            Poll::Ready(())
        } else {
            wmap.add_flow_context_f32(cx, release_time);
            Poll::Pending
        }
    })
    .await;
}

pub async fn wait_pause_entity(fe: FlowEntity, pause_time: f32) {
    let now_time = get_current_time(&fe.world());
    wait_until_entity(fe, now_time + pause_time).await;
}

pub async fn wait_until_entity(fe: FlowEntity, release_time: f32) {
    fe.poll(move |mut e, cx| {
        let mut wmap = e.world().get_resource_mut::<FlowTimerMap>().unwrap();
        if release_time <= wmap.current_time {
            Poll::Ready(())
        } else {
            wmap.add_flow_context_f32(cx, release_time);
            Poll::Pending
        }
    })
    .await;
}
