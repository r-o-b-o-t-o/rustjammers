using System;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using TMPro;
using Ui;
using UnityEngine;

namespace Main
{
	public class GameViewManagerScript : MonoBehaviour
	{
		// Author: Created by Esteban / Edited by Axel
		[SerializeField] private Transform p1Transform;
		[SerializeField] private Transform p1Hands;

		[SerializeField] private Transform p2Transform;
		[SerializeField] private Transform p2Hands;

		[SerializeField] private Transform frisbeeTransform;
		private bool frisbeeHeld;
		private int timetoend;

		[SerializeField] private TextMeshPro timer;
		[SerializeField] private TextMeshPro p1Score;
		[SerializeField] private TextMeshPro p2Score;

		private readonly HumanInput[] inputs = new HumanInput[2];
		private bool block=false;
		private AgentTypeScript agentTypeManager;
		[SerializeField] private EndScreenManager endScreenManager;

		[SerializeField] private PauseScreenManager pauseScreenManager;

		public int nbFrames=1000;
		public int nbSim=3;
		
		[StructLayout(LayoutKind.Sequential)]
		private struct ManagedState
		{
			public double p1_x;
			public double p1_y;
			public sbyte p1_score;

			public double p2_x;
			public double p2_y;
			public sbyte p2_score;

			public double zbee_x;
			public double zbee_y;
			public sbyte zbee_held;

			public double time;

			public sbyte State;
		}

		private ManagedState mState;

		[DllImport("rustjammers_engine")]
		private static extern void send_type_p1(IntPtr gameEngine, sbyte type1, double frames, int sim);

		[DllImport("rustjammers_engine")]
		private static extern void send_type_p2(IntPtr gameEngine, sbyte type2, double frames, int sim);
		
		[DllImport("rustjammers_engine")]
		private static extern IntPtr initialize();

		[DllImport("rustjammers_engine")]
		private static extern void epoch(IntPtr gameEngine, sbyte p1HAction, sbyte p2HAction);

		[DllImport("rustjammers_engine")]
		private static extern ManagedState get_state(IntPtr gameEngine);

		[DllImport("rustjammers_engine")]
		private static extern void reset(IntPtr gameEngine);

		[DllImport("rustjammers_engine")]
		private static extern void dispose(IntPtr gameEngine);

		private IntPtr currentGameEngine;

		[Flags]
		private enum HumanInput
		{
			Idle = 0,
			Up = 1,
			Down = 2,
			Left = 4,
			Right = 8,
			Throw = 16,
		}

		private enum StateOfGame
		{
			Start = 0,
			Playing = 1,
			End = 2
		}

		private void Start()
		{
			this.agentTypeManager = AgentTypeScript.Instance;

			this.endScreenManager.Disable();
			this.currentGameEngine = initialize();
			reset(this.currentGameEngine);
			this.mState = new ManagedState();
		
			send_type_p1(this.currentGameEngine, (sbyte) this.agentTypeManager.Types[0], AgentTypeScript.Instance.nbFrames1, AgentTypeScript.Instance.nbSim1);
			send_type_p2(this.currentGameEngine, (sbyte) this.agentTypeManager.Types[1], AgentTypeScript.Instance.nbFrames2, AgentTypeScript.Instance.nbSim2);
		}

		private void CollectInput(int index)
		{
			var inputManagerIndex = (index + 1).ToString();

			var horizontal = Input.GetAxisRaw("P" + inputManagerIndex + "Horizontal");
			if (horizontal < 0.0)
			{
				this.inputs[index] |= HumanInput.Left;
			} else if (horizontal > 0.0)
			{
				this.inputs[index] |= HumanInput.Right;
			}

			var vertical = Input.GetAxisRaw("P" + inputManagerIndex + "Vertical");
			if (vertical < 0.0)
			{
				this.inputs[index] |= HumanInput.Down;
			} else if (vertical > 0.0)
			{
				this.inputs[index] |= HumanInput.Up;
			}

			if (Input.GetButtonDown("P" + inputManagerIndex + "Throw"))
			{
				this.inputs[index] |= HumanInput.Throw;
			}
		}

		private void Update()
		{
			if (Input.GetKeyDown(KeyCode.Escape))
			{
				if (pauseScreenManager.isActived)
				{
					pauseScreenManager.Disable();
				}
				else
				{
					pauseScreenManager.Enable();
					this.pauseScreenManager.SetScore((int) this.mState.p1_score, (int) this.mState.p2_score);
				}
			}
			if (!pauseScreenManager.isActived )
			{
				this.inputs[0] = HumanInput.Idle;
				this.inputs[1] = HumanInput.Idle;

				for (var i = 0; i < this.agentTypeManager.Types.Length; i++)
				{
					var t = this.agentTypeManager.Types[i];
					if (t == AgentTypeScript.AgentType.Human)
					{
						this.CollectInput(i);
					}
				}

				epoch(this.currentGameEngine, (sbyte) this.inputs[0], (sbyte) this.inputs[1]);
				this.mState = get_state(this.currentGameEngine);
				if(mState.time>1.0 && !endScreenManager.isActived){
					if (this.mState.p1_score < 10)
					{
						this.p1Score.text = "0" + this.mState.p1_score;
					}
					else
					{
						this.p1Score.text = "" + this.mState.p1_score;
					}
					if (this.mState.p2_score < 10)
					{
						this.p2Score.text = "0" + this.mState.p2_score;
					}
					else
					{
						this.p2Score.text = "" + this.mState.p2_score;
					}
				}
				this.p1Transform.position =
					new Vector3((float) this.mState.p1_x, this.p1Transform.position.y, (float) this.mState.p1_y);
				this.p2Transform.position =
					new Vector3((float) this.mState.p2_x, this.p2Transform.position.y, (float) this.mState.p2_y);
				if (!this.frisbeeHeld)
				{
					if (this.mState.zbee_held == 0)
					{
						this.frisbeeTransform.parent = this.p1Hands;
						this.frisbeeTransform.localPosition = Vector3.zero;
						this.frisbeeHeld = true;
					}
					else if (this.mState.zbee_held == 1)
					{
						this.frisbeeTransform.parent = this.p2Hands;
						this.frisbeeTransform.localPosition = Vector3.zero;
						this.frisbeeHeld = true;
					}
				}
				else
				{
					if (this.mState.zbee_held == -1)
					{
						this.frisbeeTransform.parent = null;
						this.frisbeeHeld = false;
					}
				}
				if (!this.frisbeeHeld)
				{
					this.frisbeeTransform.position =
						new Vector3((float) this.mState.zbee_x, this.p2Transform.position.y, (float) this.mState.zbee_y);
				}
				if (this.mState.State != (sbyte) StateOfGame.End && mState.time>1.0 )
				{
					var roundedTime = Mathf.RoundToInt((float) this.mState.time);
					if (roundedTime < 10)
					{
						this.SetTimerText("0" + roundedTime);
					}
					else
					{
						this.SetTimerText(roundedTime.ToString());
					}
				}
				else
				{
					this.endScreenManager.Enable();
					if (!block)
					{
						this.endScreenManager.SetScore((int) this.mState.p1_score, (int) this.mState.p2_score);
						block = true;
					}
					Debug.Log(mState.p1_score+"/"+mState.p2_score);
					if (AgentTypeScript.Instance.turbo)
					{
						PlayAgain();
					}
				}
			}
		}

		private void OnDestroy()
		{
			dispose(this.currentGameEngine);
		}

		private void SetTimerText(string text)
		{
			this.timer.text = "TIME\n\n" + text;
		}

		public void PlayAgain()
		{
			if (pauseScreenManager.isActived)
			{
				pauseScreenManager.Disable();
			}
			this.endScreenManager.Disable();
			reset(this.currentGameEngine);
		}	
	}
}