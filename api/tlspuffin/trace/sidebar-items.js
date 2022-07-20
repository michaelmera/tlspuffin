initSidebarItems({"enum":[["Action","There are two action types [`OutputAction`] and [`InputAction`] differ. Both actions drive the internal state machine of an [`Agent`] forward by calling `next_state()`. The [`OutputAction`] first forwards the state machine and then extracts knowledge from the TLS messages produced by the underlying stream by calling  `take_message_from_outbound(...)`. The [`InputAction`] evaluates the recipe term and injects the newly produced message into the inbound channel of the [`Agent`] referenced through the corresponding [`Step`]s by calling `add_to_inbound(...)` and then drives the state machine forward. Therefore, the difference is that one step increases the knowledge of the attacker, whereas the other action uses the available knowledge."],["TlsMessageType","[MessageType] contains TLS-related typing information, this is to be distinguished from the *.typ fields It uses [rustls::msgs::enums::{ContentType,HandshakeType}]."]],"struct":[["InputAction","The [`InputAction`] evaluates the recipe term and injects the newly produced message into the inbound channel of the [`Agent`] referenced through the corresponding [`Step`]s by calling `add_to_inbound(...)` and then drives the state machine forward."],["Knowledge","[Knowledge] describes an atomic piece of knowledge inferred by the [`crate::variable_data::extract_knowledge`] function [Knowledge] is made of the data, the agent that produced the output, the TLS message type and the internal type."],["OutputAction","The [`OutputAction`] first forwards the state machine and then extracts knowledge from the TLS messages produced by the underlying stream by calling  `take_message_from_outbound(...)`. An output action is automatically called after each input step."],["Query",""],["Step",""],["Trace",""],["TraceContext","The [`TraceContext`] contains a list of [`VariableData`], which is known as the knowledge of the attacker. [`VariableData`] can contain data of various types like for example client and server extensions, cipher suits or session ID It also holds the concrete references to the [`Agent`]s and the underlying streams, which contain the messages which have need exchanged and are not yet processed by an output step."]],"trait":[["QueryMatcher",""]]});